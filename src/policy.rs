//! Agent boundaries: the sites and tabs a profile lets an agent use.
//!
//! These rules keep a cooperating agent inside the work a person gave it, and
//! stop mistaken or prompt-injected commands. They are not a sandbox: Chrome's
//! approval is broad, and any process running as the same OS user can reach the
//! browser directly. A separate profile is the isolation boundary.

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Which tabs an agent may control in a profile.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum TabAccess {
    /// Any tab in the browser.
    #[default]
    All,
    /// Only tabs `lsearch` opened and tabs a person granted with `tabs grant`.
    Own,
}

/// A profile's site rules and tab access.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Policy {
    /// When not empty, only these sites may be opened or read.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allow: Vec<String>,
    /// Sites that may never be opened or read.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub block: Vec<String>,
    #[serde(default)]
    pub tabs: TabAccess,
}

/// Browser-internal pages (settings, passwords, extensions, developer tools) are never
/// agent work, whatever the rules say.
const BROWSER_SCHEMES: &[&str] = &[
    "about",
    "brave",
    "chrome",
    "chrome-extension",
    "chrome-search",
    "chrome-untrusted",
    "devtools",
    "edge",
    "javascript",
    "opera",
    "view-source",
    "vivaldi",
];

/// A site rule: a domain and its subdomains (`example.com`), only the
/// subdomains (`*.example.com`), optionally narrowed to a path (`example.com/admin`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    host: String,
    subdomains_only: bool,
    path: String,
}

impl Rule {
    /// Parses a rule. Schemes, ports, query strings and fragments are ignored.
    pub fn parse(raw: &str) -> Result<Self> {
        let invalid = || {
            Error::InvalidArgument(format!(
                "site rule {raw:?} is not a domain such as example.com, *.example.com or example.com/path"
            ))
        };
        let mut rest = raw.trim();
        if let Some(index) = rest.find("://") {
            rest = &rest[index + 3..];
        }
        let rest = rest.split(['?', '#']).next().unwrap_or_default();
        let (host, path) = match rest.find('/') {
            Some(index) => (&rest[..index], &rest[index..]),
            None => (rest, ""),
        };
        let (subdomains_only, host) = match host.strip_prefix("*.") {
            Some(host) => (true, host),
            None => (false, host),
        };
        let host = host.rsplit_once(':').map_or(host, |(name, port)| {
            if port.chars().all(|c| c.is_ascii_digit()) {
                name
            } else {
                host
            }
        });
        let host = host.trim_end_matches('.');
        if host.is_empty() || host.contains('*') {
            return Err(invalid());
        }
        // The URL parser lowercases names and converts international domains to
        // the punycode form that page URLs report.
        let host = match url::Host::parse(host).map_err(|_| invalid())? {
            url::Host::Domain(domain) => domain,
            url::Host::Ipv4(address) => address.to_string(),
            url::Host::Ipv6(address) => format!("[{address}]"),
        };
        let path = path.trim_end_matches('/').to_owned();
        Ok(Self {
            host,
            subdomains_only,
            path,
        })
    }

    /// The normalized text stored in a policy and shown to people.
    pub fn canonical(&self) -> String {
        let prefix = if self.subdomains_only { "*." } else { "" };
        format!("{prefix}{}{}", self.host, self.path)
    }

    fn matches(&self, host: &str, path: &str) -> bool {
        let subdomain = host
            .strip_suffix(self.host.as_str())
            .is_some_and(|rest| rest.ends_with('.'));
        let host_matches = subdomain || (!self.subdomains_only && host == self.host);
        let path_matches = self.path.is_empty()
            || path == self.path
            || path
                .strip_prefix(self.path.as_str())
                .is_some_and(|rest| rest.starts_with('/'));
        host_matches && path_matches
    }

    fn matches_host(&self, host: &str) -> bool {
        self.path.is_empty() && self.matches(host, "")
    }
}

impl Policy {
    /// Fails with `site_blocked` when the profile does not let agents use `raw`.
    pub fn check_url(&self, raw: &str) -> Result<()> {
        match self.blocked_by(raw, true) {
            Some((site, rule)) => Err(Error::SiteBlocked { site, rule }),
            None => Ok(()),
        }
    }

    /// Like [`Policy::check_url`] for a search engine's results page. Search
    /// engines are how `search` works, so only an explicit block stops them.
    pub fn check_search_engine(&self, raw: &str) -> Result<()> {
        match self.blocked_by(raw, false) {
            Some((site, rule)) => Err(Error::SiteBlocked { site, rule }),
            None => Ok(()),
        }
    }

    /// Whether a page URL is off limits, for listings that hide rather than fail.
    pub fn allows(&self, raw: &str) -> bool {
        self.blocked_by(raw, true).is_none()
    }

    /// Whether cookies for `domain` (as reported by Chrome, with or without a
    /// leading dot) belong to a site the profile allows.
    pub fn allows_cookie_domain(&self, domain: &str) -> bool {
        let host = domain.trim_start_matches('.').to_ascii_lowercase();
        if Self::rules(&self.block).any(|rule| rule.matches_host(&host)) {
            return false;
        }
        self.allow.is_empty() || Self::rules(&self.allow).any(|rule| rule.matches(&host, ""))
    }

    /// Adds rules to `list`, normalized and without duplicates. Returns the
    /// canonical rules that were new.
    pub fn add(list: &mut Vec<String>, raw: &[String]) -> Result<Vec<String>> {
        let parsed = raw
            .iter()
            .map(|rule| Rule::parse(rule).map(|rule| rule.canonical()))
            .collect::<Result<Vec<_>>>()?;
        let mut added = Vec::new();
        for rule in parsed {
            if !list.contains(&rule) {
                list.push(rule.clone());
                added.push(rule);
            }
        }
        Ok(added)
    }

    /// Removes rules from both lists. Returns the canonical rules that were present.
    pub fn remove(&mut self, raw: &[String]) -> Result<Vec<String>> {
        let parsed = raw
            .iter()
            .map(|rule| Rule::parse(rule).map(|rule| rule.canonical()))
            .collect::<Result<Vec<_>>>()?;
        let mut removed = Vec::new();
        for rule in parsed {
            let before = self.allow.len() + self.block.len();
            self.allow.retain(|existing| existing != &rule);
            self.block.retain(|existing| existing != &rule);
            if self.allow.len() + self.block.len() < before {
                removed.push(rule);
            }
        }
        Ok(removed)
    }

    fn rules(list: &[String]) -> impl Iterator<Item = Rule> + '_ {
        // Stored rules were validated when added; skip any hand-edited line that no longer parses.
        list.iter().filter_map(|rule| Rule::parse(rule).ok())
    }

    /// Returns the site (scheme and host, never the path or query) and the
    /// reason when `raw` is off limits.
    fn blocked_by(&self, raw: &str, use_allowlist: bool) -> Option<(String, String)> {
        let Ok(url) = url::Url::parse(raw) else {
            return (use_allowlist && !self.allow.is_empty()).then(|| {
                (
                    "an unreadable address".to_owned(),
                    "not in the allowlist".to_owned(),
                )
            });
        };
        let scheme = url.scheme();
        if scheme == "about" && matches!(url.path(), "blank" | "srcdoc") {
            return None;
        }
        if BROWSER_SCHEMES.contains(&scheme) {
            return Some((
                format!("this {scheme}: page"),
                "browser pages are never agent work".to_owned(),
            ));
        }
        // blob: and filesystem: addresses belong to the page that created them.
        let inner = matches!(scheme, "blob" | "filesystem")
            .then(|| url::Url::parse(url.path()).ok())
            .flatten();
        let page = inner.as_ref().unwrap_or(&url);
        let host = page.host_str().unwrap_or_default().to_ascii_lowercase();
        let site = if host.is_empty() {
            format!("this {}: address", page.scheme())
        } else {
            host.clone()
        };
        if !host.is_empty()
            && let Some(rule) =
                Self::rules(&self.block).find(|rule| rule.matches(&host, page.path()))
        {
            return Some((site, format!("blocked: {}", rule.canonical())));
        }
        if use_allowlist
            && !self.allow.is_empty()
            && (host.is_empty()
                || !Self::rules(&self.allow).any(|rule| rule.matches(&host, page.path())))
        {
            return Some((site, "not in the allowlist".to_owned()));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy(allow: &[&str], block: &[&str]) -> Policy {
        let mut policy = Policy::default();
        Policy::add(
            &mut policy.allow,
            &allow.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
        )
        .unwrap();
        Policy::add(
            &mut policy.block,
            &block.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>(),
        )
        .unwrap();
        policy
    }

    fn reason(policy: &Policy, url: &str) -> Option<String> {
        policy.blocked_by(url, true).map(|(_, rule)| rule)
    }

    #[test]
    fn rules_are_normalized_to_one_canonical_form() {
        for (raw, canonical) in [
            ("Example.COM", "example.com"),
            ("https://example.com/", "example.com"),
            ("https://example.com:8443/admin/?q=1#x", "example.com/admin"),
            ("*.Example.com", "*.example.com"),
            ("example.com.", "example.com"),
            ("bücher.de", "xn--bcher-kva.de"),
            ("127.0.0.1:3000", "127.0.0.1"),
        ] {
            assert_eq!(Rule::parse(raw).unwrap().canonical(), canonical, "{raw}");
        }
        for raw in [
            "",
            "   ",
            "*",
            "*.",
            "exa mple.com",
            "ex*ample.com",
            "https://",
        ] {
            assert!(Rule::parse(raw).is_err(), "{raw:?} should be rejected");
        }
    }

    #[test]
    fn a_domain_rule_covers_its_subdomains_but_not_lookalikes() {
        let policy = policy(&[], &["bank.com"]);
        assert!(reason(&policy, "https://bank.com/").is_some());
        assert!(reason(&policy, "https://www.bank.com/login").is_some());
        assert!(reason(&policy, "https://notbank.com/").is_none());
        assert!(reason(&policy, "https://bank.com.evil.test/").is_none());
    }

    #[test]
    fn a_wildcard_rule_covers_only_subdomains() {
        let policy = policy(&[], &["*.example.com"]);
        assert!(reason(&policy, "https://example.com/").is_none());
        assert!(reason(&policy, "https://mail.example.com/").is_some());
    }

    #[test]
    fn a_path_rule_stops_at_segment_boundaries() {
        let policy = policy(&[], &["mail.google.com/mail"]);
        assert!(reason(&policy, "https://mail.google.com/mail").is_some());
        assert!(reason(&policy, "https://mail.google.com/mail/u/0/").is_some());
        assert!(reason(&policy, "https://mail.google.com/mailbox").is_none());
        assert!(reason(&policy, "https://mail.google.com/").is_none());
    }

    #[test]
    fn an_allowlist_admits_only_its_sites_and_blocks_still_win() {
        let policy = policy(&["docs.rs", "github.com"], &["gist.github.com"]);
        assert!(reason(&policy, "https://docs.rs/tokio").is_none());
        assert!(reason(&policy, "https://github.com/rust-lang").is_none());
        assert_eq!(
            reason(&policy, "https://example.com/").as_deref(),
            Some("not in the allowlist")
        );
        assert_eq!(
            reason(&policy, "https://gist.github.com/x").as_deref(),
            Some("blocked: gist.github.com")
        );
        assert!(reason(&policy, "data:text/html,hi").is_some());
        assert!(reason(&policy, "file:///etc/hosts").is_some());
    }

    #[test]
    fn browser_pages_are_always_blocked_and_blank_pages_always_allowed() {
        let open = Policy::default();
        for url in [
            "chrome://settings/passwords",
            "chrome://password-manager/passwords",
            "chrome-extension://abc/popup.html",
            "devtools://devtools/bundled/inspector.html",
            "view-source:https://example.com/",
            "about:settings",
            "javascript:alert(1)",
        ] {
            assert!(reason(&open, url).is_some(), "{url}");
        }
        let strict = policy(&["docs.rs"], &[]);
        assert!(reason(&strict, "about:blank").is_none());
        assert!(reason(&strict, "about:srcdoc").is_none());
        assert!(reason(&open, "data:text/html,hi").is_none());
    }

    #[test]
    fn blob_addresses_are_judged_by_the_page_that_made_them() {
        let policy = policy(&[], &["bank.com"]);
        assert!(reason(&policy, "blob:https://www.bank.com/4a9f").is_some());
        assert!(reason(&policy, "blob:https://example.com/4a9f").is_none());
    }

    #[test]
    fn errors_name_the_site_but_never_the_path_or_query() {
        let policy = policy(&[], &["bank.com"]);
        let error = policy
            .check_url("https://www.bank.com/accounts?id=123")
            .unwrap_err();
        assert_eq!(error.code(), "site_blocked");
        let message = error.to_string();
        assert!(message.contains("www.bank.com"));
        assert!(!message.contains("accounts") && !message.contains("123"));
    }

    #[test]
    fn search_engines_ignore_the_allowlist_but_not_blocks() {
        let allow_only_docs = policy(&["docs.rs"], &[]);
        assert!(
            allow_only_docs
                .check_search_engine("https://www.google.com/search?q=x")
                .is_ok()
        );
        let no_google = policy(&[], &["google.com"]);
        assert!(
            no_google
                .check_search_engine("https://www.google.com/search?q=x")
                .is_err()
        );
    }

    #[test]
    fn cookie_domains_follow_the_site_rules() {
        let policy = policy(&["github.com"], &["gist.github.com"]);
        assert!(policy.allows_cookie_domain(".github.com"));
        assert!(!policy.allows_cookie_domain("gist.github.com"));
        assert!(!policy.allows_cookie_domain(".bank.com"));
        assert!(Policy::default().allows_cookie_domain(".bank.com"));
    }

    #[test]
    fn adding_and_removing_rules_reports_only_real_changes() {
        let mut policy = Policy::default();
        let added = Policy::add(
            &mut policy.block,
            &["Bank.com".to_owned(), "bank.com/".to_owned()],
        )
        .unwrap();
        assert_eq!(added, ["bank.com"]);
        assert_eq!(policy.block, ["bank.com"]);
        let removed = policy
            .remove(&["https://bank.com".to_owned(), "other.com".to_owned()])
            .unwrap();
        assert_eq!(removed, ["bank.com"]);
        assert!(policy.block.is_empty());
    }

    #[test]
    fn an_empty_policy_serializes_to_default_tab_access_only() {
        assert_eq!(
            serde_json::to_string(&Policy::default()).unwrap(),
            r#"{"tabs":"all"}"#
        );
        let parsed: Policy = serde_json::from_str("{}").unwrap();
        assert_eq!(parsed, Policy::default());
    }
}
