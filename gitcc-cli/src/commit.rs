//! `commit` command

use std::env;

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm, Editor, Input, Select};
use gitcc_core::{Config, ConvcoMessage, StringExt};

use crate::{error, info, success, warn};

/// Commit command arguments
#[derive(Debug, Parser)]
pub struct CommitArgs {}

/// Executes the command `commit`
pub fn run(_args: CommitArgs) -> anyhow::Result<()> {
    // load the config
    let cwd = env::current_dir()?;
    let config = Config::load_from_fs(&cwd)?;
    let config = if let Some(cfg) = config {
        cfg
    } else {
        info!("using default config");
        Config::default()
    };

    // Checks that the repo is clean
    let dirty_files = gitcc_core::dirty_files(&cwd)?;
    if !dirty_files.is_empty() {
        warn!("repo is dirty:");
        for f in dirty_files {
            eprintln!("\t{f}");
        }
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("continue ?")
            .report(true)
            .default(false)
            .interact()?
        {
            false => {
                error!("aborted");
                return Ok(());
            }
            true => {}
        }
    }

    // write the commit
    let msg = open_dialogue(&config)?;
    // eprintln!("{:#?}", commit);

    // git commit
    let commit = gitcc_core::commit_changes(&cwd, &msg.to_string())?;
    success!(format!("new commit with id {}", commit.id));

    Ok(())
}

/// Asks the user to enter the commit info
fn open_dialogue(config: &Config) -> anyhow::Result<ConvcoMessage> {
    // > type
    let r#type = {
        let commit_types: Vec<_> = config
            .commit
            .types
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        let commit_types_keys: Vec<_> = config.commit.types.keys().map(|k| k.to_string()).collect();
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

    // > Short description
    let desc = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit description")
        .report(true)
        .interact()?
        .trim()
        .to_lowercase_first();

    let mut msg = ConvcoMessage {
        r#type,
        scope,
        is_breaking: false,
        desc,
        body: None,
        footer: None,
    };

    // > breaking changes
    // dev note: 'Confirm' does not remove the blinking cursor
    match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Breaking change ?")
        .report(true)
        .default(false)
        .interact()?
    {
        false => {}
        true => {
            let breaking_change_desc = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Breaking change description".to_string())
                .report(true)
                .allow_empty(true)
                .interact_text()?;
            msg.add_breaking_change(&breaking_change_desc);
        }
    }

    // > body
    match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Commit message body ?")
        .report(true)
        .default(false)
        .interact()?
    {
        false => {}
        true => {
            let text = Editor::new()
                .require_save(true)
                .trim_newlines(true)
                .edit("")?;
            msg.body = text;
        }
    }

    // > footer
    'footer_notes: loop {
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add footer note ?")
            .report(true)
            .default(false)
            .interact()?
        {
            false => break 'footer_notes,
            true => {
                let key = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Key ?".to_string())
                    .report(true)
                    .allow_empty(false)
                    .interact_text()?;
                let value = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Value ?".to_string())
                    .report(true)
                    .allow_empty(false)
                    .interact_text()?;
                msg.add_footer_note(&key, &value);
            }
        }
    }

    Ok(msg)
}
