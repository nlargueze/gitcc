//! Init command

use std::{env, path::PathBuf, process::exit};

use anyhow::bail;
use clap::Parser;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::config::Config;

/// Init command arguments
#[derive(Debug, Parser)]
pub struct InitArgs {
    /// Path to the repo directory
    #[clap(long)]
    cwd: Option<PathBuf>,
}

/// Executes the init commnad
pub fn run(args: InitArgs) -> anyhow::Result<()> {
    // Set the working directory
    if let Some(cwd) = args.cwd {
        env::set_current_dir(cwd)?;
    }

    // Load the config
    let config = Config::load()?;

    // Ask for config generation if not found
    if config.is_some() {
        println!();
        println!("{} Repo is already configured", "i".bright_cyan());
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to reset the config ?")
            .report(true)
            .default(true)
            .interact()?
        {
            true => {
                generate_configuration(false)?;
            }
            false => {
                exit(0);
            }
        };
    } else {
        eprintln!("{} No configuration found", "i".bright_cyan());
        generate_configuration(true)?;
    }

    Ok(())
}

/// Requests the config generation
pub fn generate_configuration(ask_for_confirmation: bool) -> anyhow::Result<Config> {
    if ask_for_confirmation {
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to create a default configuration ?")
            .report(true)
            .default(true)
            .interact()?
        {
            false => {
                bail!("Configuration will not be generated");
            }
            true => {}
        };
    }

    let config = Config::default();
    config.save()?;
    eprintln!("{} Generated configuration", "✔".bright_green());

    Ok(config)
}
