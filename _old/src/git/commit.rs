//! Wrappers for git commits

use std::fmt::Display;

use chrono::{DateTime, Utc};

/// A git commit
#[derive(Debug, Clone)]
pub struct GitCommit {
    /// Commit id (hash)
    pub id: String,
    /// Commit timestamp
    pub timestamp: DateTime<Utc>,
    /// Author
    pub author: String,
    /// Message
    pub message: String,
}

impl Default for GitCommit {
    fn default() -> Self {
        Self {
            id: Default::default(),
            timestamp: Utc::now(),
            author: Default::default(),
            message: Default::default(),
        }
    }
}

impl Display for GitCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "id:{}", self.id)?;
        writeln!(f, "ts:{}", self.timestamp.to_rfc3339())?;
        writeln!(f, "author:{}", self.author)?;
        write!(f, "{}", self.message)?;
        Ok(())
    }
}
