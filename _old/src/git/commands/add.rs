//! Wrappers for `git add` commands

use std::process::Command;

use crate::error::{Error, Result};

/// Wrapper for `git add -A`
pub fn git_add() -> Result<(String, String)> {
    let output = Command::new("git")
        .args(["add", "-A", "--verbose"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid stdout");
    let stderr = String::from_utf8(output.stderr).expect("Invalid stderr");

    if !output.status.success() {
        return Err(Error::InternalError(format!(
            "Failed to execute git add: {stderr}"
        )));
    }

    Ok((stdout, stderr))
}
