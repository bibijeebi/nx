use crate::config::store::Config;
use crate::utils::command::run_command;
use crate::utils::package::is_unfree_package;

pub fn install(package: &str, passthrough_args: &[String], config: &Config) -> i32 {
    if is_unfree_package(package) {
        if !config.allow_unfree {
            println!("Warning: {} is an unfree package.", package);
            println!("To allow installation, run: nx config --allow-unfree true");
            return 1;
        }
    }
    
    let cmd = format!("nixpkgs#{}", package);
    match run_command("nix", &["profile", "install", &cmd], passthrough_args, config) {
        Ok(status) => status.code().unwrap_or(1),
        Err(_) => {
            eprintln!("Failed to install package");
            1
        }
    }
}