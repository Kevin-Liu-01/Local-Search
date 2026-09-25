# Security

`local-search` controls a browser profile that may already be signed in to your
accounts. Treat the Chrome DevTools Protocol endpoint as sensitive local
authority.

- Bind debugging endpoints to localhost only.
- Do not expose `--remote-debugging-port` on a public interface.
- Avoid committing generated HAR, MHTML, HTML, screenshot, PDF, or cookie output.
- Cookie values are redacted by default; use `cookies list --show-values` only in
  private terminals.
- `request` runs `fetch` inside the browser, so it can carry ambient cookies and
  local storage credentials for the page origin.

Choose browser authority explicitly:

- `connect --existing` uses Chrome 144+'s opt-in debugging endpoint. Enable
  `chrome://inspect/#remote-debugging` yourself and approve Chrome's prompt.
  The CLI reads the selected user-data root's `DevToolsActivePort`; it never
  copies cookie databases or relaunches your everyday profile with flags.
  Chrome may ask for consent again for each new connection. Selection persistence
  does not suppress or bypass that consent.
- `connect --managed` / `launch` explicitly select a separate persistent profile
  under the local-search config directory, with loopback CDP. Sign in there only
  to accounts you want your agent to access.

Chrome 136+ ignores command-line debugging flags on its default profile. Do not
work around this by exporting credentials or weakening Chrome's security settings.
See [Chrome's configuration guide](https://developer.chrome.com/docs/devtools/agents/get-started/configuration).

The browser selection is only saved after a successful CDP round trip. Missing,
denied, stale, or unreachable selected endpoints return `browser_disconnected`,
without discovery or launch fallback. Explicit `--cdp` is transient. With no
selection, browser commands request a choice rather than silently opening Chrome.
Search caches are scoped to the browser session and checked only after connection.
Ordinary page commands create/reuse a background work tab, not an arbitrary
personal tab; `--target` and `tabs use` deliberately grant control of a named tab.

CDP approval is broad browser authority, not per-site authorization. Returned
signed-in page data goes to the calling agent and may be sent to its model provider.
Only connect accounts you trust that agent with; keep tasks within your authority.
Disable remote debugging in Chrome to revoke future existing-browser connections.

## Release checks

Interactive welcome/setup can request public version metadata from crates.io,
or registry.npmjs.org for the npm bridge. These optional checks run outside the
browser using system `curl`, with HTTPS verification, no redirects or `.curlrc`,
a bounded response size, and a two-second timeout. They send a fixed User-Agent
and package URL, not queries, cookies, page content, account identifiers, or
browser/profile data. As with any network request, the registry sees the request's
source IP; system proxy environment settings may apply.

Automatic checks use a daily local cache and never install or execute downloaded
code. They are skipped in pipelines, CI, `--json` / `--pretty` mode, and ordinary
search/browser commands. Set `LOCAL_SEARCH_NO_UPDATE_CHECK=1` to opt out of
automatic checks. The explicit `lsearch update-check` command still fetches
release metadata when requested. Update commands use fixed package names;
untrusted registry text is never interpolated into a shell command.
