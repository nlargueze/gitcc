//! Amends an exisiting commit

use std::process::exit;

use clap::Parser;

use console::{style, Term};
use gitext::{
    commands::shared::set_current_dir_from_arg,
    git::{git_add, git_commit_amend, git_push},
};
use log::debug;

/// Lint command
#[derive(Debug, Parser)]
#[clap(author, version, about = "Lints a commit message")]
pub struct Cli {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// If set, the commit will be pushed to the remote
    #[clap(long, short)]
    pub push: bool,
}

fn main() {
    env_logger::init();

    let term = Term::stderr();

    let args = Cli::parse();

    // set CWD
    let _cwd = set_current_dir_from_arg(&args.cwd);

    // git add -A
    term.write_line("Staging changes …").unwrap();
    match git_add() {
        Ok((stdout, stderr)) => {
            term.clear_last_lines(1).unwrap();
            term.write_line(
                format!("{} {}", style("✔").green(), style("Changes staged").bold()).as_str(),
            )
            .unwrap();
            if !stdout.is_empty() {
                term.write_line(&stdout).unwrap();
            }
            if !stderr.is_empty() {
                term.write_line(&stderr).unwrap();
            }
        }
        Err(err) => {
            term.clear_last_lines(1).unwrap();
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

    // git commit (amend)
    term.write_line("Amending commit …").unwrap();
    match git_commit_amend() {
        Ok(_) => {
            term.clear_last_lines(1).unwrap();
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style("Committed changes (amend)").bold()
                )
                .as_str(),
            )
            .unwrap();
        }
        Err(err) => {
            term.clear_last_lines(1).unwrap();
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

    // git push
    if args.push {
        term.write_line("Pushing …").unwrap();
        match git_push() {
            Ok(_) => {
                term.clear_last_lines(1).unwrap();
                term.write_line(
                    format!("{} {}", style("✔").green(), style("Pushed commit").bold()).as_str(),
                )
                .unwrap();
            }
            Err(_) => {
                term.clear_last_lines(1).unwrap();
                term.write_line(style("✗ Failed to push commit").red().to_string().as_str())
                    .unwrap();
                exit(1);
            }
        }
    } else {
        debug!("git push skipped");
    }
}
