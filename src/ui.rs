use std::{
    env,
    io::{self, IsTerminal, Write as _},
    sync::mpsc::{self, Sender},
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::error::IoContext as _;

const RESET: &str = "\x1b[0m";
const TEAL: &str = "\x1b[38;2;81;200;204m";
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
             {TEAL}Browser Search API{RESET}  {DIM}No API key · No billing{RESET}\n\
             {DIM}Structured web search through your local browser.{RESET}\n\n\
               {DIM}${RESET} lsearch search \"rust browser automation\" --limit 3\n\
               {DIM}${RESET} lsearch launch\n\n\
             {DIM}Run `lsearch --help` for every command.{RESET}"
        )
    } else {
        format!(
            "{MARK}\n\
             Browser Search API  No API key · No billing\n\
             Structured web search through your local browser.\n\n\
               $ lsearch search \"rust browser automation\" --limit 3\n\
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
    use super::{BRAILLE_FRAMES, MARK};

    #[test]
    fn branded_mark_and_spinner_frames_are_present() {
        assert!(MARK.contains("███████"));
        assert!(BRAILLE_FRAMES.iter().all(|frame| !frame.is_empty()));
    }
}
