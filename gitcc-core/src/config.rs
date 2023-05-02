//! Configuration

use std::{
    fs,
    path::{Path, PathBuf},
};

use gitcc_git::discover_repo;
use serde::{Deserialize, Serialize};

use crate::{error::Error, ChangelogConfig, CommitConfig, VersioningConfig};

/// Config directory name
pub const CONFIG_DIR_NAME: &str = ".gitcc";

/// Config file name
pub const CONFIG_FILE_NAME: &str = "config.toml";

/// Configuration
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    /// Commit configuration
    pub commit: CommitConfig,
    /// Versioning configuration
    pub version: VersioningConfig,
    /// Changelog configuration
    pub changelog: ChangelogConfig,
}

impl Config {
    /// Loads the configuration from a file system
    ///
    /// The configuration is looked into the parent git repo.
    pub fn load_from_fs(cwd: &Path) -> Result<Option<Self>, Error> {
        let cfg_file = Self::file_path(cwd)?;
        if cfg_file.exists() && cfg_file.is_file() {
            let data = fs::read_to_string(&cfg_file)?;
            let config = toml::from_str::<Self>(&data)?;
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }

    /// Saves the file to the file system
    ///
    /// The file is saved relative to the workdir of the parent git repo.
    pub fn save_to_fs(&self, cwd: &Path, overwrite: bool) -> Result<(), Error> {
        let cfg_file = Self::file_path(cwd)?;
        if cfg_file.exists() {
            if !overwrite {
                // do not overwrite
                return Err(Error::msg("config file already exists"));
            }
            fs::remove_file(&cfg_file)?;
        }

        let cfg_dir = cfg_file.parent().unwrap();
        if !cfg_dir.exists() {
            fs::create_dir(cfg_dir)?;
        }
        let cfg_str = toml::to_string(self)?;
        fs::write(cfg_dir.join(CONFIG_FILE_NAME), cfg_str)?;

        Ok(())
    }

    /// Returns the path to the config file
    fn file_path(cwd: &Path) -> Result<PathBuf, Error> {
        let repo = discover_repo(cwd)?;
        let repo_dir = repo
            .workdir()
            .ok_or(Error::msg("git repo workdir not found (bare repo)"))?;
        let cfg_file = repo_dir.join(CONFIG_DIR_NAME).join(CONFIG_FILE_NAME);
        Ok(cfg_file)
    }
}
