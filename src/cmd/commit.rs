//! Commit command

use std::{env, path::PathBuf};

use clap::Parser;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Input, Select};

use crate::{
    cmd::init::generate_configuration, commit::ConvCommitMessage, config::Config, git,
    util::StringExt,
};

/// Commit command arguments
#[derive(Debug, Parser)]
pub struct CommitArgs {
    /// Path to the repo directory
    #[clap(long)]
    cwd: Option<PathBuf>,
    /// If set, the commit will be pushed to the remote
    #[clap(long, short)]
    push: bool,
}

/// Executres the commit commnad
pub fn run(args: CommitArgs) -> anyhow::Result<()> {
    // Set the working directory
    if let Some(cwd) = args.cwd {
        env::set_current_dir(cwd)?;
    }

    // Load the config
    let mut config = Config::load()?;

    // Ask for config generation
    if config.is_none() {
        eprintln!("{} No configuration found", "i".bright_cyan());
        config = Some(generate_configuration(true)?);
    }
    let config = config.unwrap();

    // Write the commit
    let commit = ask_for_commit_message(&config)?;

    // git add -A
    let stdout = git::add()?;
    eprintln!("{} Changes staged", "✔".bright_green());
    eprintln!("{}", stdout);

    // git commit
    let commit_msg = commit.to_string();
    let stdout = git::commit(&commit_msg)?;
    eprintln!("{} Committed changes", "✔".bright_green());
    eprintln!("{}", stdout);

    // git push
    if args.push {
        let stdout = git::push(false)?;
        eprintln!("{} Pushed commit", "✔".bright_green());
        eprintln!("{}", stdout);
    }

    Ok(())
}

/// Asks for the commit message
fn ask_for_commit_message(config: &Config) -> anyhow::Result<ConvCommitMessage> {
    // > type
    let r#type = {
        let commit_types: Vec<_> = config
            .commits
            .types
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        let commit_types_keys: Vec<_> = config
            .commits
            .types
            .iter()
            .map(|(k, _v)| k.to_string())
            .collect();
        let i = Select::with_theme(&ColorfulTheme::default())
            .items(&commit_types)
            .clear(true)
            .default(0)
            .report(true)
            .with_prompt("Commit type")
            .interact()?;

        commit_types_keys[i].clone()
    };

    // > scope
    let scope = {
        let scope: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Commit scope")
            .report(true)
            .allow_empty(true)
            .interact_text()?;
        if scope.is_empty() {
            None
        } else {
            Some(scope.to_lowercase())
        }
    };

    // > subject
    let subject = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit subject")
        .report(true)
        .interact()?
        .trim()
        .to_lowercase_first();

    // > body
    let body = {
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Commit message body ?")
            .report(true)
            .default(false)
            .interact()?
        {
            false => None,
            true => Editor::new()
                .executable("micro")
                .require_save(true)
                .trim_newlines(true)
                .edit("")?,
        }
    };

    // > breaking changes
    let breaking_change = {
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Breaking change ?")
            .report(true)
            .default(false)
            .interact()?
        {
            false => None,
            true => Some(
                Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Breaking change description".to_string())
                    .report(true)
                    .allow_empty(true)
                    .interact_text()?,
            ),
        }
    };

    // > closed issues
    let closed_issues = {
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Closes issues ?")
            .report(true)
            .default(false)
            .interact()?
        {
            false => None,
            true => {
                let issues_str = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Closed issues (comma separated) ?".to_string())
                    .report(true)
                    .allow_empty(true)
                    .interact_text()?;
                match issues_str.as_str() {
                    "" => None,
                    s => Some(
                        s.split(',')
                            .enumerate()
                            .map(|(_i, p)| p.trim().parse::<u32>())
                            .collect::<Result<Vec<_>, _>>()?,
                    ),
                }
            }
        }
    };

    Ok(ConvCommitMessage {
        r#type,
        scope,
        subject,
        body,
        breaking_change,
        closed_issues,
    })
}
