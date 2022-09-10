//! Bumps the version.

use std::process::exit;

use clap::Parser;

use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm};
use gitext::{
    commands::shared::{load_config, set_current_dir_from_arg},
    git::{git_set_tag, git_status_porcelain},
    version::get_repo_next_version,
};

/// Bump command
#[derive(Debug, Parser)]
#[clap(author, version, about = "Bumps the version")]
pub struct Cli {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// Allows uncommitted changes when setting the tag
    #[clap(long)]
    pub allow_dirty: bool,
    /// If set, the repo is tagged with the new version
    #[clap(long)]
    pub tag: bool,
}

fn main() {
    env_logger::init();

    let term = Term::stderr();

    let args = Cli::parse();

    // set CWD
    let cwd = set_current_dir_from_arg(&args.cwd);

    // load the config
    let config = load_config(&cwd, true);

    // check if the repo is pristine
    if !args.allow_dirty {
        let commit_status = match git_status_porcelain() {
            Ok(status) => status,
            Err(err) => {
                term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                    .unwrap();
                exit(1);
            }
        };

        if let Some(files_list) = commit_status {
            term.write_line(
                style("> Repo has uncommited changes:")
                    .bold()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            term.write_line(&files_list).unwrap();
            let ok = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Tag despite uncommited changes ?")
                .report(true)
                .default(false)
                .interact()
                .unwrap();
            if !ok {
                term.write_line(
                    style("✗ Uncommited changes -> skipped".to_string())
                        .red()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                exit(0);
            }
        }
    }

    // bump
    let (next_version, curr_version) = match get_repo_next_version(&config) {
        Ok(commits) => commits,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };
    term.write_line(
        format!(
            "{} {}",
            style("i").yellow(),
            style(format!(
                "{} --> {}",
                curr_version
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "<none>".to_string()),
                next_version
            ))
            .bold(),
        )
        .as_str(),
    )
    .unwrap();

    let next_git_version = format!("v{}", next_version);

    // dry run
    if !args.tag {
        term.write_line(
            format!(
                "{} {}",
                style("i").yellow(),
                style("Git tag not set").bold().bold(),
            )
            .as_str(),
        )
        .unwrap();
        print!("{}", next_git_version);
        exit(0);
    }

    // Tag the repo with the new version
    match git_set_tag(
        &next_git_version,
        format!("Version {next_git_version}").as_str(),
    ) {
        Ok(_) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style(format!("Repo tagged as {next_git_version}")).bold()
                )
                .as_str(),
            )
            .unwrap();
        }
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

    print!("{}", next_git_version);
}
