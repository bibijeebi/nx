use crate::config::store::Config;
use crate::utils::command::run_command;

pub fn execute(all: bool, package: Option<String>, passthrough_args: &[String], config: &Config) -> i32 {
    if all {
        match run_command("nix", &["profile", "upgrade", ".*"], passthrough_args, config) {
            Ok(status) => status.code().unwrap_or(1),
            Err(_) => {
                eprintln!("Failed to update all packages");
                1
            }
        }
    } else if let Some(pkg) = package {
        match run_command("nix", &["profile", "upgrade", &pkg], passthrough_args, config) {
            Ok(status) => status.code().unwrap_or(1),
            Err(_) => {
                eprintln!("Failed to update package");
                1
            }
        }
    } else {
        eprintln!("Either --all or package must be specified");
        1
    }
}