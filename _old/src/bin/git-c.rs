//! Commits a conventional commit message

use std::{num::ParseIntError, process::exit};

use clap::Parser;

use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Input, Select};
use gitext::{
    commands::shared::{load_config, set_current_dir_from_arg},
    conventional::ConventionalCommitMessage,
    git::{git_add, git_commit, git_push},
    utils::StringExt,
};
use log::debug;

/// git-c command
#[derive(Debug, Parser)]
#[clap(author, version, about = "Commits a conventional commit message")]
pub struct Cli {
    /// Path to the repo directory
    #[clap(long)]
    pub cwd: Option<String>,
    /// If set, the commit will be pushed to the remote
    #[clap(long, short)]
    pub push: bool,
    /// If set, no commit is made
    #[clap(long)]
    pub dry_run: bool,
}

fn main() {
    env_logger::init();

    let term = Term::stderr();

    let args = Cli::parse();

    // set CWD
    let cwd = set_current_dir_from_arg(&args.cwd);

    // load the config
    let config = load_config(&cwd, true);

    // git commit
    // > type
    let r#type = {
        let commit_types: Vec<_> = config
            .commit
            .types
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        let commit_types_keys: Vec<_> = config
            .commit
            .types
            .iter()
            .map(|(k, _v)| k.to_string())
            .collect();
        let select_type = Select::with_theme(&ColorfulTheme::default())
            .items(&commit_types)
            .clear(true)
            .default(0)
            .report(true)
            .with_prompt("Commit type")
            .interact_on_opt(&Term::stderr())
            .unwrap();
        let r#type = match select_type {
            Some(i) => commit_types_keys[i].clone(),
            None => {
                term.write_line(
                    style("✗ A commit type must be selected")
                        .red()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                exit(1);
            }
        };
        r#type
    };

    // > subject
    let subject = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit subject")
        .report(true)
        .interact_text()
        .unwrap()
        .trim()
        .to_lowercase_first();

    // > scope
    let scope = {
        let scope: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Commit scope")
            .report(true)
            .allow_empty(true)
            .interact_text()
            .unwrap();
        if scope.is_empty() {
            None
        } else {
            Some(scope.to_lowercase())
        }
    };

    // > body
    let body = {
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Commit message body ?")
            .report(true)
            .default(false)
            .interact()
            .unwrap()
        {
            false => None,
            true => {
                match Editor::new()
                    .executable("micro")
                    .require_save(true)
                    .trim_newlines(true)
                    .edit("")
                {
                    Ok(b) => b,
                    Err(err) => {
                        term.write_line(
                            style(format!("✗ Failed to edit body: {err}"))
                                .red()
                                .to_string()
                                .as_str(),
                        )
                        .unwrap();
                        exit(1);
                    }
                }
            }
        }
    };

    // > breaking changes
    let breaking_change = {
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Breaking change ?")
            .report(true)
            .default(false)
            .interact()
            .unwrap()
        {
            false => None,
            true => Some(
                Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Breaking change description".to_string())
                    .report(true)
                    .allow_empty(true)
                    .interact_text()
                    .unwrap(),
            ),
        }
    };

    // > closed issues
    let closed_issues = {
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Closes issues ?")
            .report(true)
            .default(false)
            .interact()
            .unwrap()
        {
            false => None,
            true => {
                let issues_str = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Closed issues (comma separated) ?".to_string())
                    .report(true)
                    .allow_empty(true)
                    .interact_text()
                    .unwrap();
                match issues_str.as_str() {
                    "" => None,
                    s => {
                        let issues: Result<Vec<_>, ParseIntError> = s
                            .split(',')
                            .enumerate()
                            .map(|(_i, p)| p.trim().parse::<u32>())
                            .collect();
                        match issues {
                            Ok(ids) => Some(ids),
                            Err(err) => {
                                term.write_line(
                                    style(format!("✗ Invalid issue id: {err}"))
                                        .red()
                                        .to_string()
                                        .as_str(),
                                )
                                .unwrap();
                                exit(1);
                            }
                        }
                    }
                }
            }
        }
    };

    // write the commit message
    let commit = ConventionalCommitMessage {
        r#type,
        scope,
        subject,
        body,
        breaking_change,
        closed_issues,
    };

    let commit_msg = commit.to_string();

    // validate the commit message
    match ConventionalCommitMessage::parse(&commit_msg, &config.valid_commit_types()) {
        Ok(_) => {}
        Err(err) => {
            term.write_line(style(format!("✗ {err}")).red().to_string().as_str())
                .unwrap();
            exit(1);
        }
    }

    // dry-run
    if args.dry_run {
        term.write_line("--- COMMIT MESSAGE ---").unwrap();
        term.write_line(&commit_msg).unwrap();
        exit(0);
    }

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

    // submit the commit
    term.write_line("Committing …").unwrap();
    match git_commit(&commit_msg) {
        Ok(_) => {
            term.clear_last_lines(1).unwrap();
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style("Committed changes").bold()
                )
                .as_str(),
            )
            .unwrap();
            // print the commit message
            term.write_line(&commit_msg).unwrap();
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
            Err(err) => {
                term.clear_last_lines(1).unwrap();
                term.write_line(
                    style(format!("✗ Failed to push: {err}"))
                        .red()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                exit(1);
            }
        };
    } else {
        debug!("git push skipped");
    }
}
