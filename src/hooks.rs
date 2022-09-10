//! Git hooks

use std::collections::BTreeMap;

use anyhow::bail;

/// Hooks configuration
///
/// The key is the git hook, the value is the command to run
pub type HooksConfig = BTreeMap<String, Vec<String>>;

/// Creates the git hook shell scripts
pub fn create_git_hooks_scripts(config: &HooksConfig) -> anyhow::Result<BTreeMap<String, String>> {
    let mut scripts: BTreeMap<String, String> = BTreeMap::new();

    for (key, commands) in config {
        if !matches!(
            key.as_str(),
            "pre-commit" | "prepare-commit-msg" | "commit-msg" | "post-commit" | "pre-push"
        ) {
            bail!("Invalid hook {}", key);
        }

        let mut script = r"#!/bin/sh".to_string();
        script.push('\n');
        script.push('\n');
        script.push_str(format!("echo 'i Running {key} hook'").as_str());
        script.push('\n');
        script.push('\n');
        for cmd in commands {
            script.push_str(cmd);
            script.push('\n');
        }

        scripts.insert(key.to_string(), script.to_string());
    }

    Ok(scripts)
}
