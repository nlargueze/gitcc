//! Errors

/// Crate error
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("TOML serialization error: {0}")]
    TomlSerializeError(#[from] toml::ser::Error),
    #[error("TOML deserialization error: {0}")]
    TomlDeserializeError(#[from] toml::de::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    InvalidCommit(String),
    #[error("{0}")]
    Semver(#[from] semver::Error),
    #[error("{0}")]
    InternalError(String),
    #[error("{0}")]
    NoCommits(String),
    #[error("Template error: {0}")]
    TemplateError(#[from] handlebars::TemplateError),
    #[error("Template render error: {0}")]
    TemplateRenderError(#[from] handlebars::RenderError),
    #[error("Invalid hook: {0}")]
    InvalidHook(String),
}

/// Crate result type
pub type Result<T> = std::result::Result<T, Error>;
