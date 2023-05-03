//! Error

/// Core error
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(String);

impl Error {
    /// Creates an [Error] from a string
    pub fn msg(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::msg(&value.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Error::msg(&value.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Error::msg(&value.to_string())
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self {
        Error::msg(&value.to_string())
    }
}

impl From<gitcc_git::Error> for Error {
    fn from(value: gitcc_git::Error) -> Self {
        Error::msg(&value.to_string())
    }
}

impl From<gitcc_convco::ConvcoError> for Error {
    fn from(value: gitcc_convco::ConvcoError) -> Self {
        Error::msg(&value.to_string())
    }
}
