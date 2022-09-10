//! Git commands

use std::process::Command;

use anyhow::bail;

/// Wrapper for `git add -A`
///
/// Returns stdout as the result
pub fn add() -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["add", "-A", "--verbose"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if !output.status.success() {
        bail!(stderr);
    }

    Ok(stdout)
}

/// Wrapper for `git commit`
///
/// Returns stdout as the result
pub fn commit(msg: &str) -> anyhow::Result<String> {
    let mut cmd = Command::new("git");
    cmd.args(["commit", "-m", msg]);
    let output = cmd.output().expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if !output.status.success() {
        bail!(stderr);
    }

    Ok(stdout)
}

/// Wrapper for `git push`
pub fn push() -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["push"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if !output.status.success() {
        bail!(stderr);
    }

    Ok(stdout)
}

/// Wrapper for `git push --follow-tags`
pub fn push_follow_tags() -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["push", "--follow-tags"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    if !output.status.success() {
        bail!(stderr);
    }

    Ok(stdout)
}
