use std::env;

use gitext::{commands::shared::load_config, version::exec_bump_commands};

#[test]
fn test_exec_bump_commands() {
    let config = load_config(&env::current_dir().unwrap(), false);
    let commands = exec_bump_commands(&config, "0.1.0").unwrap();
    eprintln!("{:?}", commands);
}
