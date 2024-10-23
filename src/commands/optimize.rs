use crate::config::store::Config;
use crate::utils::command::{run_command, run_command_background};

pub fn execute(foreground: bool, passthrough_args: &[String], config: &Config) -> i32 {
    if foreground {
        match run_command("nix", &["store", "optimize-store"], passthrough_args, config) {
            Ok(status) => status.code().unwrap_or(1),
            Err(_) => {
                eprintln!("Failed to optimize store");
                1
            }
        }
    } else {
        run_command_background("nix", &["store", "optimize-store"], passthrough_args);
        0
    }
}