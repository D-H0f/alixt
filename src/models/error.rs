use thiserror::Error;


#[derive(Error, Debug)]
pub enum AlixtError {
    #[error("Failed to read file")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse TOML content")]
    Toml(#[from] toml::de::Error),

    #[error("HTTP request failed")]
    Request(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}
