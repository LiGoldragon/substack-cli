//! Substack CLI: create, publish, and manage Substack posts via the private web API.

pub mod application;
pub mod cli;
pub mod client;
pub mod error;
pub mod image_file;
pub mod prosemirror;
pub mod table_image;
pub mod types;

pub use application::{Application, ApplicationConfig};
pub use cli::CommandLine;
pub use client::Client;
pub use error::Error;
