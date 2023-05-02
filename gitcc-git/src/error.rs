//! Git error

#[derive(Debug, thiserror::Error)]
#[error("git error: {0}")]
pub struct Error(String);

impl Error {
    /// Cretes an [Error] from a string
    pub fn msg(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl From<git2::Error> for Error {
    fn from(value: git2::Error) -> Self {
        Error(value.message().to_string())
    }
}
