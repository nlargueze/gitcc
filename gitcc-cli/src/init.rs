//! `init` command

use std::env;

use clap::Parser;
use colored::Colorize;

use dialoguer::{theme::ColorfulTheme, Confirm};
use gitcc_core::Config;

/// Init command arguments
#[derive(Debug, Parser)]
pub struct InitArgs {}

/// Executes the commnad `init`
pub fn run(_args: InitArgs) -> anyhow::Result<()> {
    let cwd = env::current_dir()?;
    let config = Config::load_from_fs(&cwd)?;

    if config.is_some() {
        println!("{} repo already has a config", "i".blue().bold());
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Overwrite ?")
            .report(true)
            .default(false)
            .interact()?
        {
            false => {
                println!("{} config not recreated", "i".blue().bold());
                return Ok(());
            }
            true => {}
        }
    } else {
        println!("{} no config found", "i".blue().bold());
    }

    let config = Config::default();
    config.save_to_fs(&cwd, true)?;
    eprintln!("{} created default config", "✔".green().bold());
    Ok(())
}
