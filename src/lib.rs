//! Local browser control and structured search primitives.

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

use cli::Cli;
use error::Result;

/// Executes one parsed CLI request.
///
/// # Errors
/// Returns a typed error when browser discovery, transport, page execution, or
/// artifact persistence fails.
pub async fn run(cli: Cli) -> Result<()> {
    commands::run(cli).await
}
