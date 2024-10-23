use crate::config::store::Config;
use crate::utils::command::{run_command, run_command_background};

pub fn execute(
    delete_old: bool,
    foreground: bool,
    passthrough_args: &[String],
    config: &Config,
) -> i32 {
    let gc_args = if delete_old { vec!["-d"] } else { vec![] };
    if foreground {
        match run_command("nix-collect-garbage", &gc_args, passthrough_args, config) {
            Ok(status) => status.code().unwrap_or(1),
            Err(_) => {
                eprintln!("Failed to collect garbage");
                1
            }
        }
    } else {
        run_command_background("nix-collect-garbage", &gc_args, passthrough_args);
        0
    }
}
