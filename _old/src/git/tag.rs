//! Wrappers for git tags

use std::fmt::Display;

use chrono::{DateTime, Utc};

/// Git tag
#[derive(Debug, Clone, Eq)]
pub struct GitTag {
    /// Tag name
    pub tag: String,
    /// Tag hash
    pub hash: String,
    /// Tag commmit hash
    pub commit_hash: String,
    /// Tag date
    pub date: DateTime<Utc>,
    /// Tag message
    pub message: Option<String>,
}

impl PartialEq for GitTag {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl Display for GitTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.tag,
            self.message
                .clone()
                .map(|m| format!(" | {}", m))
                .unwrap_or_default()
        )
    }
}
