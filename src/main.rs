mod application;
mod cli;
mod client;
mod error;
mod image_file;
mod prosemirror;
mod types;

use std::process::ExitCode;

use application::{Application, ApplicationConfig};
use cli::CommandLine;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .with_writer(std::io::stderr)
        .init();

    let command_line = CommandLine::read();
    let config = match ApplicationConfig::from_environment() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };
    let application = Application::new(config.client(), config.hostname().to_string());

    match application.run(command_line).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
