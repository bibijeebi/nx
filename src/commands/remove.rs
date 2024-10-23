use crate::config::store::Config;
use crate::utils::command::run_command;

pub fn execute(
    all: bool,
    package: Option<String>,
    passthrough_args: &[String],
    config: &Config,
) -> i32 {
    if all {
        match run_command(
            "sh",
            &[
                "-c",
                "nix profile list | awk '{print $4}' | xargs -I{} nix profile remove {}",
            ],
            passthrough_args,
            config,
        ) {
            Ok(status) => status.code().unwrap_or(1),
            Err(_) => {
                eprintln!("Failed to remove all packages");
                1
            }
        }
    } else if let Some(pkg) = package {
        match run_command(
            "nix",
            &["profile", "remove", &pkg],
            passthrough_args,
            config,
        ) {
            Ok(status) => status.code().unwrap_or(1),
            Err(_) => {
                eprintln!("Failed to remove package");
                1
            }
        }
    } else {
        eprintln!("Either --all or package must be specified");
        1
    }
}
