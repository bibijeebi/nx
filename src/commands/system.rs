use std::fs;
use std::path::{Path, PathBuf};
use crate::config::store::Config;

const NIXOS_CONFIG_PATH: &str = "/etc/nixos/configuration.nix";
const TEMP_CONFIG_PATH: &str = "/etc/nixos/.nx-temp.nix";

pub fn execute(command: crate::cli::SystemCommands, passthrough_args: &[String], config: &Config) -> i32 {
    match command {
        crate::cli::SystemCommands::Package { package, permanent } => {
            if permanent {
                add_system_package(&package)
            } else {
                add_temp_package(&package)
            }
        },
        crate::cli::SystemCommands::Enable { program, permanent } => {
            let option = format!("{}.enable", program);
            if permanent {
                set_nixos_option(&option, "true")
            } else {
                set_temp_option(&option, "true")
            }
        },
        crate::cli::SystemCommands::SetOption { path, value, permanent } => {
            if permanent {
                set_nixos_option(&path, &value)
            } else {
                set_temp_option(&path, &value)
            }
        },
        crate::cli::SystemCommands::Apply => {
            apply_temp_changes()
        },
        crate::cli::SystemCommands::Show => {
            show_pending_changes()
        },
    }
}

fn get_config_path() -> PathBuf {
    PathBuf::from(NIXOS_CONFIG_PATH)
}

fn get_temp_path() -> PathBuf {
    PathBuf::from(TEMP_CONFIG_PATH)
}

fn ensure_temp_config() -> std::io::Result<()> {
    let temp_path = get_temp_path();
    if !temp_path.exists() {
        fs::write(&temp_path, "# Temporary NixOS configuration changes\n{ config, pkgs, ... }:\n{\n}\n")?;
    }
    Ok(())
}

fn add_system_package(package: &str) -> i32 {
    let config_path = get_config_path();
    if let Err(e) = modify_config_file(&config_path, |content| {
        add_package_to_config(content, package)
    }) {
        eprintln!("Failed to modify configuration: {}", e);
        return 1;
    }
    
    println!("Added {} to system packages. Run 'sudo nixos-rebuild switch' to apply.", package);
    0
}

fn add_temp_package(package: &str) -> i32 {
    if let Err(e) = ensure_temp_config() {
        eprintln!("Failed to create temporary config: {}", e);
        return 1;
    }

    let temp_path = get_temp_path();
    if let Err(e) = modify_config_file(&temp_path, |content| {
        add_package_to_config(content, package)
    }) {
        eprintln!("Failed to modify temporary configuration: {}", e);
        return 1;
    }

    println!("Added {} to temporary configuration. Run 'nx sys apply' to apply changes.", package);
    0
}

fn set_nixos_option(path: &str, value: &str) -> i32 {
    let config_path = get_config_path();
    if let Err(e) = modify_config_file(&config_path, |content| {
        add_option_to_config(content, path, value)
    }) {
        eprintln!("Failed to modify configuration: {}", e);
        return 1;
    }

    println!("Set {} = {}. Run 'sudo nixos-rebuild switch' to apply.", path, value);
    0
}

fn set_temp_option(path: &str, value: &str) -> i32 {
    if let Err(e) = ensure_temp_config() {
        eprintln!("Failed to create temporary config: {}", e);
        return 1;
    }

    let temp_path = get_temp_path();
    if let Err(e) = modify_config_file(&temp_path, |content| {
        add_option_to_config(content, path, value)
    }) {
        eprintln!("Failed to modify temporary configuration: {}", e);
        return 1;
    }

    println!("Set {} = {} in temporary configuration. Run 'nx sys apply' to apply changes.", path, value);
    0
}

fn apply_temp_changes() -> i32 {
    let temp_path = get_temp_path();
    if !temp_path.exists() {
        println!("No temporary changes to apply.");
        return 0;
    }

    // Apply changes using nixos-rebuild
    let status = std::process::Command::new("sudo")
        .args(["nixos-rebuild", "switch", "-I", &format!("nixos-config={}", temp_path.display())])
        .status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Successfully applied temporary changes.");
                0
            } else {
                eprintln!("Failed to apply changes.");
                1
            }
        }
        Err(e) => {
            eprintln!("Failed to run nixos-rebuild: {}", e);
            1
        }
    }
}

fn show_pending_changes() -> i32 {
    let temp_path = get_temp_path();
    if !temp_path.exists() {
        println!("No pending changes.");
        return 0;
    }

    match fs::read_to_string(&temp_path) {
        Ok(content) => {
            println!("Pending changes in temporary configuration:");
            println!("{}", content);
            0
        }
        Err(e) => {
            eprintln!("Failed to read temporary configuration: {}", e);
            1
        }
    }
}

fn modify_config_file<F>(path: &Path, modifier: F) -> std::io::Result<()>
where
    F: FnOnce(&str) -> String,
{
    let content = fs::read_to_string(path)?;
    let new_content = modifier(&content);
    fs::write(path, new_content)?;
    Ok(())
}

fn add_package_to_config(content: &str, package: &str) -> String {
    if content.contains("environment.systemPackages = with pkgs; [") {
        // Add package to existing system packages list
        content.replace(
            "environment.systemPackages = with pkgs; [",
            &format!("environment.systemPackages = with pkgs; [\n    {}", package),
        )
    } else {
        // Create new system packages list
        format!(
            "{}\n  environment.systemPackages = with pkgs; [\n    {}\n  ];\n}}",
            &content[..content.rfind('}').unwrap_or(content.len())],
            package
        )
    }
}

fn add_option_to_config(content: &str, path: &str, value: &str) -> String {
    let option_line = format!("  {} = {};\n", path, value);
    
    // Add the option before the closing brace
    if let Some(pos) = content.rfind('}') {
        format!("{}{}{}", &content[..pos], option_line, &content[pos..])
    } else {
        format!("{}{}}}", content, option_line)
    }
}