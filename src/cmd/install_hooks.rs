//! install-hooks command

use std::{env, fs, path::PathBuf};

use anyhow::bail;
use clap::Parser;
use colored::Colorize;

use crate::{config::Config, git, hook::get_hook_scripts};

/// install-hooks command arguments
#[derive(Debug, Parser)]
pub struct InstallHooksArgs {
    /// Path to the repo directory
    #[clap(long)]
    cwd: Option<PathBuf>,
}

/// Install git hooks
pub fn run(args: InstallHooksArgs) -> anyhow::Result<()> {
    // Set the working directory
    if let Some(cwd) = args.cwd {
        env::set_current_dir(cwd)?;
    }

    // Load the config
    let config = match Config::load()? {
        Some(c) => c,
        None => bail!("No configuration found"),
    };

    // Create the hooks scripts
    let scripts = get_hook_scripts(&config.hooks)?;

    // Write the hooks to the config folder
    let hooks_dir = config.hooks_dir();
    if hooks_dir.exists() {
        fs::remove_dir_all(&hooks_dir)?;
    }
    fs::create_dir(&hooks_dir)?;
    eprintln!(
        "{} Created hooks in {}",
        "✔".bright_green(),
        hooks_dir.display()
    );
    for (key, script) in scripts {
        let script_file = hooks_dir.join(&key);
        fs::write(&script_file, script)?;
        eprintln!("{} Generated script for hook {}", "✔".bright_green(), key);
    }

    // Configure git
    let hooks_dir_short = hooks_dir.strip_prefix(&config.repo_dir)?;
    git::config_hooks_path(hooks_dir_short)?;
    eprintln!(
        "{} Added .repo/hooks folder to git config core.hooksPath",
        "✔".bright_green()
    );

    Ok(())
}
