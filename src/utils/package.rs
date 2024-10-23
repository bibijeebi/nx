use std::process::Command;

pub fn is_unfree_package(package: &str) -> bool {
    let output = Command::new("nix")
        .args(["eval", &format!("nixpkgs#{package}.meta.license.free")])
        .output()
        .ok();
    
    if let Some(output) = output {
        String::from_utf8_lossy(&output.stdout).trim() == "false"
    } else {
        false
    }
}