//! `init` command

use std::env;

use clap::Parser;

use dialoguer::{theme::ColorfulTheme, Confirm};
use gitcc_core::Config;

use crate::{error, info, new_line, success, warn};

/// Init command arguments
#[derive(Debug, Parser)]
pub struct InitArgs {}

/// Executes the commnad `init`
pub fn run(_args: InitArgs) -> anyhow::Result<()> {
    let cwd = env::current_dir()?;
    let config = Config::load_from_fs(&cwd)?;

    new_line!();
    if config.is_some() {
        warn!("repo already has a config");
        match Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("overwrite ?")
            .report(true)
            .default(false)
            .interact()?
        {
            false => {
                error!("config not recreated");
                return Ok(());
            }
            true => {}
        }
    } else {
        info!("config not found");
    }

    let config = Config::default();
    config.save_to_fs(&cwd, true)?;
    success!("created default config");
    Ok(())
}
