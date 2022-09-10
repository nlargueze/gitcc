//! Shared utilities for commands

use std::{
    env::{current_dir, set_current_dir},
    path::{Path, PathBuf},
    process::exit,
};

use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm};
use log::debug;

use crate::config::Config;

/// Sets the current directory from an argument
pub fn set_current_dir_from_arg(cwd_input: &Option<String>) -> PathBuf {
    let term = Term::stderr();

    let cwd = match current_dir() {
        Ok(path) => path,
        Err(err) => {
            term.write_line(
                style(format!("✗ Internal error: {err}"))
                    .red()
                    .bold()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            exit(1);
        }
    };

    let cwd = if let Some(arg_cwd) = &cwd_input {
        cwd.join(arg_cwd)
    } else {
        cwd
    };

    // set the current directory
    match set_current_dir(&cwd) {
        Ok(_) => {
            debug!("Current directory set to {}", cwd.display());
        }
        Err(err) => {
            term.write_line(
                style(format!("✗ Failed to set current directory: {err}"))
                    .red()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            exit(1);
        }
    };

    cwd
}

/// Loads the configuration recursively from the current directory
pub fn load_config(cwd: &Path, ask_for_creation: bool) -> Config {
    let term = Term::stderr();

    // recursive lookup
    let mut cfg: Option<Config> = None;
    let mut currdir = cwd.to_owned();
    'rec: while cfg.is_none() {
        match Config::load(&currdir) {
            Ok(c) => match c {
                Some(x) => {
                    cfg = Some(x);
                }
                None => {
                    let popped = currdir.pop();
                    if !popped {
                        break 'rec;
                    }
                }
            },
            Err(err) => {
                term.write_line(
                    style(format!("✗ Error loading config : {err}"))
                        .red()
                        .to_string()
                        .as_str(),
                )
                .unwrap();
                exit(1);
            }
        }
    }

    if let Some(c) = cfg {
        return c;
    }

    if !ask_for_creation {
        term.write_line(
            style("✗ Config not found".to_string())
                .red()
                .to_string()
                .as_str(),
        )
        .unwrap();
        exit(1);
    }

    let cfg = match Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Create default config ?")
        .report(true)
        .default(true)
        .interact()
        .unwrap()
    {
        false => {
            term.write_line(
                style("✗ Config not found".to_string())
                    .red()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            exit(1);
        }
        true => Config::default(),
    };

    match cfg.save(cwd) {
        Ok(_) => {
            term.write_line(
                format!(
                    "{} {}",
                    style("✔").green(),
                    style("Generated config file").bold()
                )
                .as_str(),
            )
            .unwrap();
        }
        Err(err) => {
            term.write_line(
                style(format!("✗ Failed to create config file: {}", err))
                    .red()
                    .bold()
                    .to_string()
                    .as_str(),
            )
            .unwrap();
            exit(1);
        }
    }

    cfg
}
