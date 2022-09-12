//! Hook scripts

use std::collections::BTreeMap;

use anyhow::bail;
use indoc::formatdoc;

/// Hooks configuration
///
/// - key = git hook type
/// - value = script(s)
pub type HooksConfig = BTreeMap<String, Vec<String>>;

/// Valid git hooks
const VALID_GIT_HOOKS: [&str; 5] = [
    "pre-commit",
    "prepare-commit-msg",
    "commit-msg",
    "post-commit",
    "pre-push",
];

/// Creates the git hook shell scripts
///
/// The key is the git hook name, and the value is the script.
pub fn get_hook_scripts(config: &HooksConfig) -> anyhow::Result<BTreeMap<String, String>> {
    let mut scripts: BTreeMap<String, String> = BTreeMap::new();

    for (key, commands) in config {
        let key = key.to_string();

        if !VALID_GIT_HOOKS.contains(&key.as_str()) {
            bail!("Invalid git hook {}", key);
        }

        let script = formatdoc! {"
        #!/bin/sh

        echo 'i Running hook {key}';
        {lines}
        ",
        lines = commands.join("\n"),
        };

        scripts.insert(key, script);
    }

    Ok(scripts)
}
