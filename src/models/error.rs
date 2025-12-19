use thiserror::Error;


#[derive(Error, Debug)]
pub enum AlixtError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse TOML content: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Failed to parse JSON body: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP request failed")]
    Request(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal Application error: {0}")]
    InternalError(String),
}
