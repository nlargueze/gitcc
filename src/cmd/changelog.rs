//! Changelog command

use std::{env, path::PathBuf};

use anyhow::bail;
use clap::Parser;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::{
    changelog::{Changelog, ChangelogFormat},
    commit::Commits,
    config::Config,
    git,
};

/// Changelog command arguments
#[derive(Debug, Parser)]
pub struct ChangelogArgs {
    /// Path to the repo directory
    #[clap(long)]
    cwd: Option<PathBuf>,
    /// Allows uncommitted changes when setting the tag
    #[clap(long)]
    pub allow_dirty: bool,
}

/// Generates the change log
pub fn run(args: ChangelogArgs) -> anyhow::Result<()> {
    // Set the working directory
    if let Some(cwd) = args.cwd {
        env::set_current_dir(cwd)?;
    }

    // Load the config
    let config = match Config::load()? {
        Some(c) => c,
        None => bail!("No configuration found"),
    };

    // Check if the repo is pristine
    if !args.allow_dirty {
        let uncommitted = git::status_porcelain()?;

        if let Some(stdout) = uncommitted {
            eprintln!("{} Repo has uncommited changes", "i".bright_cyan());
            eprintln!("{}", stdout);

            match Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Repo has uncommited changes, continue ?")
                .report(true)
                .default(false)
                .interact()?
            {
                true => {}
                false => {
                    bail!("Uncommited changes -> skipped");
                }
            }
        }
    }

    // Generate the changelog
    let commits = Commits::load(&config.commits)?;
    let changelog = Changelog::new(&config.changelog, &commits, false)?;
    let changelog_str = changelog.generate(ChangelogFormat::Full)?;
    eprintln!("{} Changelog generated", "✔".bright_green());
    println!("{}", changelog_str);

    Ok(())
}
