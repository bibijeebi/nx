use crate::config::store::Config;
use crate::utils::command::run_command;

pub fn execute(query: &str, passthrough_args: &[String], config: &Config) -> i32 {
    match run_command("nix", &["search", "nixpkgs", query], passthrough_args, config) {
        Ok(status) => status.code().unwrap_or(1),
        Err(_) => {
            eprintln!("Failed to search packages");
            1
        }
    }
}