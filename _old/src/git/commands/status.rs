//! Wrapper for `git status` commands.

use std::process::Command;

use crate::error::{Error, Result};

/// Wrapper for `git status --porcelain`
///
/// Returns a list of files that are pending to be committed.
pub fn git_status_porcelain() -> Result<Option<String>> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid stdout");
    let stderr = String::from_utf8(output.stderr).expect("Invalid stderr");

    if !output.status.success() {
        return Err(Error::InternalError(format!(
            "Failed to execute git add: {stderr}"
        )));
    }

    if stdout.is_empty() {
        Ok(None)
    } else {
        Ok(Some(stdout))
    }
}
