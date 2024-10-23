pub mod fetcher;
pub mod expression;

use crate::config::store::Config;
use std::fs;
use std::path::Path;

pub async fn execute(url: &str, output: Option<String>, _passthrough_args: &[String], _config: &Config) -> i32 {
    match fetcher::generate_expression(url).await {
        Ok(expression) => {
            match output {
                Some(path) => {
                    if let Err(e) = fs::write(Path::new(&path), expression) {
                        eprintln!("Failed to write to file: {}", e);
                        return 1;
                    }
                    println!("Generated Nix expression written to: {}", path);
                }
                None => {
                    println!("Generated Nix expression:");
                    println!("{}", expression);
                }
            }
            0
        }
        Err(e) => {
            eprintln!("Failed to generate expression: {}", e);
            1
        }
    }
}