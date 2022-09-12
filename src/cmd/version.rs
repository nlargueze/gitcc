//! Version command

use std::{env, path::PathBuf};

use anyhow::bail;
use clap::Parser;
use colored::Colorize;

use crate::{commit::Commits, config::Config, git};

/// Bump command arguments
#[derive(Debug, Parser)]
pub struct BumpArgs {
    /// Path to the repo directory
    #[clap(long)]
    cwd: Option<PathBuf>,
}

/// Gets the current version and determines the next version
pub fn run(args: BumpArgs) -> anyhow::Result<()> {
    // Set the working directory
    if let Some(cwd) = args.cwd {
        env::set_current_dir(cwd)?;
    }

    // Load the config
    let config = match Config::load()? {
        Some(c) => c,
        None => bail!("No configuration found"),
    };

    // Load the commits
    let commits = Commits::load(&config.commits)?;

    // Get the current release tag
    let release_tag = commits.latest_release_tag()?;
    eprintln!(
        "{} Current version: {}",
        "✔".bright_green(),
        match &release_tag {
            Some(tag) => tag,
            None => "--no version--",
        }
    );

    // Check for uncommited changes
    let uncommitted = git::status_porcelain()?;
    if let Some(stdout) = uncommitted {
        eprintln!("{} Repo has uncommited changes", "i".bright_cyan());
        eprintln!("{}", stdout);
    }

    // Get the next version
    let next_version = commits.next_version()?;
    eprintln!("{} Next version: {}", "✔".bright_green(), next_version);

    Ok(())
}
