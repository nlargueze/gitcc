//! Release command

use std::{env, fs, path::PathBuf};

use anyhow::{anyhow, bail};
use clap::Parser;
use colored::Colorize;

use crate::{
    changelog::{Changelog, ChangelogFormat},
    commit::Commits,
    config::Config,
    git,
    release::execute_bump_commands,
};

/// Release command arguments
#[derive(Debug, Parser)]
pub struct ReleaseArgs {
    /// Path to the repo directory
    #[clap(long)]
    cwd: Option<PathBuf>,
    /// Allows uncommitted changes when creating the release
    #[clap(long)]
    pub allow_dirty: bool,
    /// Skips the commit
    #[clap(long)]
    pub no_commit: bool,
    /// Push the changes
    #[clap(long, short)]
    pub push: bool,
}

/// Creates a release
pub fn run(args: ReleaseArgs) -> anyhow::Result<()> {
    // Set the working directory
    if let Some(cwd) = args.cwd {
        env::set_current_dir(cwd)?;
    }

    // Load the config
    let config = match Config::load()? {
        Some(c) => c,
        None => bail!("No configuration found"),
    };

    // Check if the repo is dirty
    let uncommitted = git::status_porcelain()?;
    if let Some(stdout) = uncommitted {
        eprintln!("{} Repo has uncommited changes", "i".bright_cyan());
        eprintln!("{}", stdout);
        if !args.allow_dirty {
            bail!("Uncommited changes -> exited");
        }
    }

    // Create the changelog
    let commits = Commits::load(&config.commits)?;
    let changelog = Changelog::new(&config.changelog, &commits, true)?;

    let latest_version = changelog
        .latest_version()
        .ok_or_else(|| anyhow!("Missing version"))?;
    eprintln!("{} New version: {}", "✔".bright_green(), latest_version);

    let changelog_str = changelog.generate(ChangelogFormat::Full)?;
    fs::write(config.repo_dir.join("CHANGELOG.md"), changelog_str)?;
    eprintln!("{} Changelog generated", "✔".bright_green());

    let releasenotes_str = changelog.generate(ChangelogFormat::LatestRelease)?;
    fs::write(config.repo_dir.join("RELEASE_NOTES.md"), releasenotes_str)?;
    eprintln!("{} Release notes generated", "✔".bright_green());

    // Execute bump commands
    let exec_cmds = execute_bump_commands(&config, &latest_version)?;
    if !exec_cmds.is_empty() {
        eprintln!("{} Executed bump commands", "✔".bright_green());
        for (cmd, stdout) in &exec_cmds {
            eprintln!("> {}", cmd);
            eprintln!("{}", stdout);
        }
    }

    // Skip if flag `no-commit` is set
    if args.no_commit {
        return Ok(());
    }

    // Add changes and create new commit
    let _git_add_out = git::add()?;
    eprintln!("{} Staged changes", "✔".bright_green());
    eprintln!("{}", _git_add_out);

    let commit_msg = format!("chore: release {latest_version}");
    let _git_commit_out = git::commit(&commit_msg)?;
    eprintln!("{} Committed", "✔".bright_green());
    eprintln!("{}", _git_commit_out);

    // // Tag the commit
    // let x = git::tag(tag, msg)?;

    // // Push the commit
    // let x = git::push()?;

    Ok(())
}
