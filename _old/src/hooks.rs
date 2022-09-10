//! Git hooks management

use std::collections::HashMap;

use crate::{
    config::Config,
    error::{Error, Result},
};

/// Creates the git hook shell scripts
pub fn create_git_hooks_scripts(config: &Config) -> Result<HashMap<String, String>> {
    let mut scripts: HashMap<String, String> = HashMap::new();

    for (key, commands) in &config.hooks {
        if !matches!(
            key.as_str(),
            "pre-commit" | "prepare-commit-msg" | "commit-msg" | "post-commit" | "pre-push"
        ) {
            return Err(Error::InvalidHook(key.to_string()));
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
