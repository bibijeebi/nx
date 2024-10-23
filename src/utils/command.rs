use crate::config::store::Config;
use std::process::{Command, Stdio};

pub fn run_command(
    program: &str,
    args: &[&str],
    passthrough_args: &[String],
    config: &Config,
) -> std::io::Result<std::process::ExitStatus> {
    let mut command = Command::new(program);
    command.args(args);
    if !passthrough_args.is_empty() {
        command.args(passthrough_args);
    }

    if config.allow_unfree {
        command.env("NIXPKGS_ALLOW_UNFREE", "1");
        if !passthrough_args.contains(&String::from("--impure")) {
            command.arg("--impure");
        }
    }

    command.status()
}

pub fn run_command_background(program: &str, args: &[&str], passthrough_args: &[String]) {
    let mut command = Command::new(program);
    command
        .args(args)
        .args(passthrough_args)
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    if let Ok(_child) = command.spawn() {
        println!("Process started in background");
    } else {
        eprintln!("Failed to start background process");
    }
}
