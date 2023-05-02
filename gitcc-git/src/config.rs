//! Configuration

use std::collections::BTreeMap;

use crate::{error::Error, repo::GitRepository};

/// A more user-friendly config object
pub type GitConfig = BTreeMap<String, String>;

/// Config key for the user email
pub const USER_EMAIL: &str = "user.email";

/// Config key for the user name
pub const USER_NAME: &str = "user.name";

/// Returns the repo config
///
/// The config is the config which can be applied taking into
/// account the hierarchy which exists between the global config,
/// and the local/repo config overwriting the global config.
pub fn repo_config(repo: &GitRepository) -> Result<GitConfig, Error> {
    let git2_cfg = repo.config().unwrap();

    let mut cfg = BTreeMap::new();
    git2_cfg.entries(None).into_iter().for_each(|entries| {
        // NB: this loop is over the types of config, starting from the most generic one (global => local)
        entries
            .for_each(|entry| {
                // NB: this loop is for each entry in the config file
                // let level = entry.level();
                let name = entry.name().unwrap().to_string();
                let value = entry.value().unwrap_or("").to_string();
                // eprintln!("{:?} {} {}", level, name, value);
                cfg.insert(name, value);
            })
            .unwrap();
    });

    Ok(cfg)
}

/// Retrieves the repo origin url
pub fn repo_origin_url(repo: &GitRepository, origin_name: &str) -> Result<Option<String>, Error> {
    let cfg = repo_config(repo)?;
    let key = format!("remote.{}.url", origin_name);
    Ok(cfg.get(key.as_str()).map(|s| s.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::repo::discover_repo;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_cfg() {
        let cwd = std::env::current_dir().unwrap();
        let repo = discover_repo(&cwd).unwrap();
        let cfg = repo_config(&repo).unwrap();
        for (k, v) in cfg.iter() {
            eprintln!("{}: {}", k, v);
        }
    }
}
