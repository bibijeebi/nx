use crate::config::store::Config;
use crate::utils::command::run_command;

pub fn execute(passthrough_args: &[String], config: &Config) -> i32 {
    match run_command("nix", &["profile", "list"], passthrough_args, config) {
        Ok(status) => status.code().unwrap_or(1),
        Err(_) => {
            eprintln!("Failed to list packages");
            1
        }
    }
}
