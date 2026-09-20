use std::{
    env,
    io::{self, IsTerminal, Write as _},
    sync::mpsc::{self, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};

use serde_json::Value;

use crate::error::IoContext as _;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const UNDERLINE: &str = "\x1b[4m";
const TEAL: &str = "\x1b[38;2;81;200;204m";
const BLUE: &str = "\x1b[38;2;122;183;255m";
const GREEN: &str = "\x1b[38;2;91;196;139m";
const DIM: &str = "\x1b[38;2;127;136;143m";

const MARK: &str = r#"   ██╗ ██╗      ███████╗
  ██╔╝ ██║      ██╔════╝
 ██╔╝  ██║      ███████╗
██╔╝   ██║      ╚════██║
██║    ███████╗ ███████║
╚═╝    ╚══════╝ ╚══════╝"#;

const BRAILLE_FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Prints the branded human entry point used when no query or command is supplied.
pub fn print_welcome() -> crate::error::Result<()> {
    let stdout = io::stdout();
    let color = color_enabled(stdout.is_terminal());
    let mut out = stdout.lock();
    let rendered = if color {
        format!(
            "{TEAL}{MARK}{RESET}\n\
             {TEAL}Local Browser API for Agents{RESET}\n\
             {DIM}Search · read · use signed-in sites{RESET}\n\n\
               {DIM}${RESET} lsearch \"rust browser automation\" --engine google --limit 3\n\
               {DIM}${RESET} lsearch read https://example.com --format json\n\
               {DIM}${RESET} lsearch launch\n\n\
             {DIM}Run `lsearch --help` for every command.{RESET}"
        )
    } else {
        format!(
            "{MARK}\n\
             Local Browser API for Agents\n\
             Search · read · use signed-in sites\n\n\
               $ lsearch \"rust browser automation\" --engine google --limit 3\n\
               $ lsearch read https://example.com --format json\n\
               $ lsearch launch\n\n\
             Run `lsearch --help` for every command."
        )
    };
    writeln!(out, "{rendered}").at("stdout")
}

/// A compact braille spinner that never contaminates stdout or piped JSON.
pub struct Spinner {
    stop: Option<Sender<()>>,
    worker: Option<JoinHandle<()>>,
    color: bool,
}

impl Spinner {
    pub fn start(label: impl Into<String>) -> Self {
        let is_terminal = io::stderr().is_terminal();
        if !motion_enabled(is_terminal) {
            return Self {
                stop: None,
                worker: None,
                color: false,
            };
        }

        let label = label.into();
        let color = color_enabled(is_terminal);
        let (stop, receiver) = mpsc::channel();
        let worker = thread::spawn(move || {
            let mut frame = 0_usize;
            eprint!("\x1b[?25l");
            loop {
                let glyph = BRAILLE_FRAMES[frame % BRAILLE_FRAMES.len()];
                if color {
                    eprint!("\r\x1b[2K{TEAL}{glyph}{RESET} {label}");
                } else {
                    eprint!("\r\x1b[2K{glyph} {label}");
                }
                let _ = io::stderr().flush();
                frame += 1;
                match receiver.recv_timeout(Duration::from_millis(72)) {
                    Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                }
            }
            eprint!("\r\x1b[2K\x1b[?25h");
            let _ = io::stderr().flush();
        });

        Self {
            stop: Some(stop),
            worker: Some(worker),
            color,
        }
    }

    pub fn success(mut self, message: impl AsRef<str>) {
        if self.stop_worker() {
            if self.color {
                eprintln!("{GREEN}✓{RESET} {}", message.as_ref());
            } else {
                eprintln!("✓ {}", message.as_ref());
            }
        }
    }

    fn stop_worker(&mut self) -> bool {
        let active = self.worker.is_some();
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
        active
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        let _ = self.stop_worker();
    }
}

pub fn success(message: impl AsRef<str>) {
    let is_terminal = io::stderr().is_terminal();
    if motion_enabled(is_terminal) {
        if color_enabled(is_terminal) {
            eprintln!("{GREEN}✓{RESET} {}", message.as_ref());
        } else {
            eprintln!("✓ {}", message.as_ref());
        }
    }
}

/// Prints a concise, colored search-result view for people using a terminal.
pub fn print_search_results(query: &str, engine: &str, search: &Value) -> crate::error::Result<()> {
    let stdout = io::stdout();
    let is_terminal = stdout.is_terminal();
    let width = terminal_width();
    let rendered = render_search_results(
        query,
        engine,
        search,
        color_enabled(is_terminal),
        hyperlinks_enabled(is_terminal),
        width,
    );
    let mut out = stdout.lock();
    writeln!(out, "{rendered}").at("stdout")
}

fn render_search_results(
    query: &str,
    engine: &str,
    search: &Value,
    color: bool,
    hyperlinks: bool,
    width: usize,
) -> String {
    let results = search
        .get("results")
        .and_then(Value::as_array)
        .map_or(&[][..], Vec::as_slice);
    let blocked = search
        .get("blocked")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let content_width = width.saturating_sub(4).max(40);
    let mut lines = Vec::with_capacity(results.len().saturating_mul(4).saturating_add(3));
    let result_label = if results.len() == 1 {
        "result"
    } else {
        "results"
    };

    if color {
        lines.push(format!(
            "{TEAL}{engine}{RESET} {DIM}·{RESET} {BOLD}{} {result_label}{RESET} {DIM}for “{}”{RESET}",
            results.len(),
            truncate(
                query,
                content_width.saturating_sub(engine.chars().count() + 20)
            )
        ));
    } else {
        lines.push(format!(
            "{engine} · {} {result_label} for “{}”",
            results.len(),
            truncate(
                query,
                content_width.saturating_sub(engine.chars().count() + 20)
            )
        ));
    }

    if blocked {
        lines.push(if color {
            format!("{DIM}Search engine verification may have limited these results.{RESET}")
        } else {
            "Search engine verification may have limited these results.".to_owned()
        });
    }

    for (index, result) in results.iter().enumerate() {
        let rank = result
            .get("rank")
            .and_then(Value::as_u64)
            .unwrap_or((index + 1) as u64);
        let title = field(result, "title", "Untitled result");
        let domain = field(result, "domain", "unknown domain");
        let url = field(result, "url", "");
        let snippet = field(result, "snippet", "");
        let title = truncate(title, content_width);
        // Keep the fallback URL complete so terminals without OSC 8 support can
        // still recognize it as a native link, even when it wraps visually.
        let visible_url = terminal_text(url);
        let title = hyperlink(&title, url, hyperlinks);
        let visible_url = hyperlink(&visible_url, url, hyperlinks);

        lines.push(String::new());
        if color {
            lines.push(format!("{TEAL}{rank:02}{RESET}  {BOLD}{title}{RESET}"));
            lines.push(format!(
                "    {BLUE}{}{RESET}",
                truncate(domain, content_width)
            ));
            lines.push(format!("    {BLUE}{UNDERLINE}{visible_url}{RESET}"));
            if !snippet.is_empty() {
                lines.push(format!(
                    "    {DIM}{}{RESET}",
                    truncate(snippet, content_width)
                ));
            }
        } else {
            lines.push(format!("{rank:02}  {title}"));
            lines.push(format!("    {}", truncate(domain, content_width)));
            lines.push(format!("    {visible_url}"));
            if !snippet.is_empty() {
                lines.push(format!("    {}", truncate(snippet, content_width)));
            }
        }
    }

    lines.join("\n")
}

fn field<'a>(value: &'a Value, key: &str, fallback: &'a str) -> &'a str {
    value.get(key).and_then(Value::as_str).unwrap_or(fallback)
}

fn terminal_width() -> usize {
    env::var("COLUMNS")
        .ok()
        .and_then(|columns| columns.parse::<usize>().ok())
        .unwrap_or(100)
        .clamp(60, 140)
}

fn hyperlinks_enabled(is_terminal: bool) -> bool {
    is_terminal
        && env::var_os("LOCAL_SEARCH_PLAIN").is_none()
        && env::var("TERM").map_or(true, |term| term != "dumb")
}

fn hyperlink(label: &str, target: &str, enabled: bool) -> String {
    if !enabled || !safe_link_target(target) {
        return label.to_owned();
    }
    format!("\x1b]8;;{target}\x1b\\{label}\x1b]8;;\x1b\\")
}

fn safe_link_target(target: &str) -> bool {
    !target.chars().any(char::is_control)
        && url::Url::parse(target).is_ok_and(|url| matches!(url.scheme(), "http" | "https"))
}

fn truncate(value: &str, width: usize) -> String {
    let value = terminal_text(value);
    if value.chars().count() <= width {
        return value;
    }
    let take = width.saturating_sub(1);
    format!("{}…", value.chars().take(take).collect::<String>())
}

fn terminal_text(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_control())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn motion_enabled(is_terminal: bool) -> bool {
    is_terminal
        && env::var_os("LOCAL_SEARCH_PLAIN").is_none()
        && env::var("TERM").map_or(true, |term| term != "dumb")
}

fn color_enabled(is_terminal: bool) -> bool {
    motion_enabled(is_terminal) && env::var_os("NO_COLOR").is_none()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{BRAILLE_FRAMES, MARK, hyperlink, render_search_results, truncate};

    #[test]
    fn branded_mark_and_spinner_frames_are_present() {
        assert!(MARK.contains("███████"));
        assert!(BRAILLE_FRAMES.iter().all(|frame| !frame.is_empty()));
    }

    #[test]
    fn human_search_results_are_compact_and_ranked() {
        let search = json!({
            "blocked": false,
            "results": [{
                "rank": 1,
                "title": "A useful result",
                "domain": "example.com",
                "url": "https://example.com/useful",
                "snippet": "A concise explanation of the result."
            }]
        });

        let rendered = render_search_results("useful query", "Google", &search, false, false, 100);

        assert!(rendered.contains("Google · 1 result for “useful query”"));
        assert!(rendered.contains("01  A useful result"));
        assert!(rendered.contains("    example.com\n    https://example.com/useful"));
        assert!(!rendered.contains("\"results\""));
    }

    #[test]
    fn colored_search_results_use_distinct_styles() {
        let search = json!({
            "results": [{ "title": "Result", "domain": "example.com", "url": "https://example.com" }]
        });

        let rendered = render_search_results("query", "Bing", &search, true, false, 80);

        assert!(rendered.contains("\x1b[38;2;81;200;204m"));
        assert!(rendered.contains("\x1b[38;2;122;183;255m"));
        assert!(rendered.contains("\x1b[1mResult"));
    }

    #[test]
    fn truncation_preserves_unicode_boundaries() {
        assert_eq!(truncate("abc🦀def", 6), "abc🦀d…");
    }

    #[test]
    fn human_result_titles_and_urls_are_clickable() {
        let search = json!({
            "results": [{
                "title": "Clickable result",
                "domain": "example.com",
                "url": "https://example.com/a/very/long/path"
            }]
        });

        let rendered = render_search_results("query", "Google", &search, false, true, 60);

        assert_eq!(
            rendered
                .matches("\x1b]8;;https://example.com/a/very/long/path\x1b\\")
                .count(),
            2
        );
        assert!(rendered.contains("Clickable result\x1b]8;;\x1b\\"));
        assert!(rendered.contains("https://example.com/a/very/long/path\x1b]8;;\x1b\\"));
    }

    #[test]
    fn hyperlinks_reject_non_web_and_control_character_targets() {
        assert_eq!(hyperlink("unsafe", "javascript:alert(1)", true), "unsafe");
        assert_eq!(
            hyperlink("unsafe", "https://example.com/\u{1b}escape", true),
            "unsafe"
        );
    }

    #[test]
    fn fallback_urls_are_complete_and_terminal_text_is_sanitized() {
        let search = json!({
            "results": [{
                "title": "Safe\u{1b} title",
                "domain": "example.com",
                "url": "https://example.com/a/very/long/path/that/exceeds/the/terminal/width"
            }]
        });

        let rendered = render_search_results("query", "Google", &search, false, false, 60);

        assert!(
            rendered
                .contains("https://example.com/a/very/long/path/that/exceeds/the/terminal/width")
        );
        assert!(!rendered.contains('\u{1b}'));
    }
}
