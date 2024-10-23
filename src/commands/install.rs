use std::process::Command;
use crate::config::store::Config;
use crate::utils::command::run_command;
use crate::utils::package::is_unfree_package;
use crate::commands::system::{add_system_package, modify_config_file};

pub fn install(package: &str, passthrough_args: &[String], config: &Config) -> i32 {
    // Check if package is unfree
    if is_unfree_package(package) {
        if !config.allow_unfree {
            println!("Warning: {} is an unfree package.", package);
            println!("To allow installation, run: nx config --allow-unfree true");
            return 1;
        }
    }

    // First, install to profile for immediate use
    println!("Installing {} to profile for immediate use...", package);
    let cmd = format!("nixpkgs#{}", package);
    let profile_result = run_command(
        "nix",
        &["profile", "install", &cmd],
        passthrough_args,
        config,
    );

    if profile_result.is_err() {
        eprintln!("Failed to install package to profile");
        return 1;
    }

    // Add to system packages
    println!("Adding {} to system packages...", package);
    if add_system_package(package) != 0 {
        eprintln!("Failed to add package to system configuration");
        return 1;
    }

    // Run nixos-rebuild switch
    println!("Rebuilding system configuration...");
    let rebuild_result = Command::new("sudo")
        .args(["nixos-rebuild", "switch"])
        .status();

    match rebuild_result {
        Ok(status) if status.success() => {
            // Remove from profile since it's now in system packages
            println!("Removing {} from profile...", package);
            if let Ok(profile_list) = Command::new("nix")
                .args(["profile", "list"])
                .output()
            {
                let output = String::from_utf8_lossy(&profile_list.stdout);
                if let Some(package_id) = find_package_id(&output, package) {
                    let remove_result = run_command(
                        "nix",
                        &["profile", "remove", &package_id],
                        &[],
                        config,
                    );
                    
                    if remove_result.is_err() {
                        eprintln!("Warning: Failed to remove package from profile");
                    }
                }
            }
            
            println!("Successfully installed {} and added to system packages!", package);
            0
        }
        _ => {
            eprintln!("Failed to rebuild system");
            1
        }
    }
}

fn find_package_id(profile_output: &str, package_name: &str) -> Option<String> {
    for line in profile_output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 && parts[3].contains(package_name) {
            return Some(parts[3].to_string());
        }
    }
    None
}