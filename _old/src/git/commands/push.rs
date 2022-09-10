//! Wrappers for `git push` commands

use std::process::Command;

use crate::error::{Error, Result};

/// Wrapper for `git push`
pub fn git_push() -> Result<()> {
    let output = Command::new("git")
        .args(["push"])
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        return Err(Error::InternalError(
            "Failed to execute git push".to_string(),
        ));
    }
    Ok(())
}

/// Wrapper for `git push --follow-tags`
pub fn git_push_follow_tags() -> Result<()> {
    let output = Command::new("git")
        .args(["push", "--follow-tags"])
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        return Err(Error::InternalError(
            "Failed to execute git push".to_string(),
        ));
    }
    Ok(())
}
