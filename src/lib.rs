//! Local search and browser-backed structured retrieval primitives.

#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::needless_raw_string_hashes,
    clippy::too_many_lines
)]

pub mod browser;
pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod output;
pub mod ui;
mod updates;

use cli::Cli;
use error::Result;

/// Executes one parsed CLI request.
///
/// # Errors
/// Returns a typed error when browser discovery, transport, page execution, or
/// artifact persistence fails.
pub async fn run(cli: Cli) -> Result<()> {
    let check_updates = updates::automatic_check_allowed(&cli);
    commands::run(cli).await?;
    if check_updates {
        updates::recommend().await;
    }
    Ok(())
}
