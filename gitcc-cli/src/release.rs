//! `release` command

use std::{env, fs, process::exit};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm};
use gitcc_core::{ChangelogBuildOptions, Config, StatusShow, TEMPLATE_CHANGELOG_STD};

use crate::{error, info, success, warn};

/// Commit command arguments
#[derive(Debug, Parser)]
pub struct ReleaseArgs {
    /// Dry run mode
    #[arg(long)]
    pub dry_run: bool,
}

/// Executes the command `release`
pub fn run(args: ReleaseArgs) -> anyhow::Result<()> {
    // load the config
    let cwd = env::current_dir()?;
    let cfg_file = Config::load_from_fs(&cwd)?;
    let cfg = if let Some(cfg) = cfg_file {
        cfg
    } else {
        info!("using default config");
        Config::default()
    };

    // make sure there is no untracked/uncommitted changes
    let dirty_files = gitcc_core::git_status(&cwd, StatusShow::IndexAndWorkdir)?;
    if !dirty_files.is_empty() {
        warn!("repo is dirty");
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("continue ?")
            .report(true)
            .default(false)
            .interact()?
        {
            true => {}
            false => {
                exit(1);
            }
        }
    }

    // find the next version
    let commit_history = gitcc_core::commit_history(&cwd, &cfg)?;
    let next_version = commit_history.next_version_str();
    info!(format!("next version: {}", next_version));

    // build the changelog
    let changelog = gitcc_core::build_changelog(
        &cwd,
        &cfg,
        &commit_history,
        Some(ChangelogBuildOptions {
            origin_name: None,
            all: false,
            next_version: Some(next_version.clone()),
        }),
    )?;
    let changelog_str = match changelog.render(TEMPLATE_CHANGELOG_STD) {
        Ok(s) => s,
        Err(err) => {
            error!(format!("failed to generate the changelog: {err}"));
            exit(1);
        }
    };
    if !args.dry_run {
        let root_dir = gitcc_core::get_root_dir(&cwd).expect("not a git repo");
        match fs::write(root_dir.join("CHANGELOG.md"), changelog_str) {
            Ok(_ok) => {
                success!("changelog written to file")
            }
            Err(err) => {
                error!(format!("failed to write the changelog: {err}"));
                exit(1);
            }
        }
    } else {
        info!("(dry-run) changelog not written to file")
    }

    // bump the packages versions
    if !args.dry_run {
        match gitcc_core::exec_bump_commands(&cfg, &next_version) {
            Ok(commands) => {
                for cmd in commands {
                    eprintln!("executed bump command: {cmd}");
                }
                success!("executed bump commands")
            }
            Err(err) => {
                error!(format!("failed to bump packages: {err}"));
                exit(1);
            }
        }
    } else {
        info!("(dry-run) skipped executing bump commands")
    }

    // commit the changes
    if !args.dry_run {
        match gitcc_core::add_all_changes(&cwd) {
            Ok(_ok) => {}
            Err(err) => {
                error!(format!("failed to add the changes: {err}"));
                exit(1);
            }
        }
        match gitcc_core::commit_changes(&cwd, &format!("chore(release): Release {next_version}")) {
            Ok(_commit) => {
                success!("commited changes");
            }
            Err(err) => {
                error!(format!("failed to commit: {err}"));
                exit(1);
            }
        }
    } else {
        info!("(dry-run) changes not committed");
    }

    // tagging the commit
    if !args.dry_run {
        match gitcc_core::set_annotated_tag(&cwd, &next_version, &format!("Release {next_version}"))
        {
            Ok(_ok) => {
                success!(format!("tag {} added", next_version));
            }
            Err(err) => {
                error!(format!("failed to add the changes: {err}"));
                exit(1);
            }
        }
    } else {
        info!(format!("(dry-run) tag '{}' not set", next_version));
    }

    // Other steps
    warn!("=> Push the changes with: git push --follow-tags");
    warn!("=> Create the github release");
    warn!("=> Publish the updated packages (crates.io, npm, brew, etc...)");

    Ok(())
}
