use std::process::ExitCode;

use clap::Parser;
use local_search::{cli::Cli, output::render_error};

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let pretty = cli.pretty;

    match local_search::run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", render_error(&error, pretty));
            ExitCode::FAILURE
        }
    }
}
