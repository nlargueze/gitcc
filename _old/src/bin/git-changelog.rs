//! Generates the changelog

use std::process::exit;

use clap::Parser;

use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm};
use gitext::{
    changelog::ChangeLog,
    commands::shared::{load_config, set_current_dir_from_arg},
    git::git_status_porcelain,
    version::get_repo_next_version,
};

/// Lint command
#[derive(Debug, Parser)]
#[clap(author, version, about = "Lints a commit message")]
pub struct Cli {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// Allows uncommitted changes when setting the tag
    #[clap(long)]
    pub allow_dirty: bool,
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
                .with_prompt("Generate changelog despite uncommited changes ?")
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

    // init the changelog
    let changelog = match ChangeLog::init() {
        Ok(cl) => cl,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // get the latest version based on the commit history
    // NB: can be replaced by Unreleased tag
    let next_version = match get_repo_next_version(&config) {
        Ok((v, _)) => v.to_string(),
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // generate the change log file
    let (changelog_str, _) = match changelog.generate(&config, &next_version) {
        Ok(s) => s,
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    };

    // print changelog
    print!("{changelog_str}");
}
