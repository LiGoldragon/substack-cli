use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Http(#[from] reqwest::Error),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Usage(String),

    #[error("{variable} must be set")]
    MissingEnvironmentVariable { variable: &'static str },

    #[error("{0}")]
    UnexpectedResponse(String),

    #[error("no publication user found")]
    NoUser,

    #[error("invalid image: {0}")]
    InvalidImage(String),

    #[error("unsupported image format: {extension}")]
    UnsupportedImageFormat { extension: String },

    #[error("API {status}: {body}")]
    Api { status: u16, body: String },
}
