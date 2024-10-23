use std::fs;
use std::path::Path;

mod expression;
mod fetcher;

use expression::{
    generate_git_expression, generate_gitea_expression, generate_github_expression,
    generate_gitlab_expression, generate_url_expression,
};

use crate::config::store::Config;
use fetcher::{analyze_url, calculate_hash, FetcherType};

pub async fn execute(
    url: &str,
    output: Option<String>,
    _passthrough_args: &[String],
    _config: &Config,
) -> i32 {
    match generate_expression(url).await {
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

async fn generate_expression(url_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let fetcher_info = analyze_url(url_str)?;
    let hash = calculate_hash(&fetcher_info).await?;

    match fetcher_info.fetcher_type {
        FetcherType::GitHub => generate_github_expression(&fetcher_info, &hash).await,
        FetcherType::URL => generate_url_expression(&fetcher_info, &hash),
        FetcherType::Git => generate_git_expression(&fetcher_info, &hash),
        FetcherType::GitLab => generate_gitlab_expression(&fetcher_info, &hash).await,
        FetcherType::FromGitea => generate_gitea_expression(&fetcher_info, &hash),
    }
}
