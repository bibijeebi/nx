use reqwest;
use sha2::{Sha256, Digest};
use url::Url;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubRepo {
    name: String,
    description: Option<String>,
    default_branch: String,
}

#[derive(Debug)]
pub enum FetcherType {
    GitHub,
    GitLab,
    URL,
    Git,
    FromGitea,
}

#[derive(Debug)]
pub struct FetcherInfo {
    pub url: String,
    pub fetcher_type: FetcherType,
    pub hash: Option<String>,
    pub version: Option<String>,
}

pub async fn generate_expression(url_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let fetcher_info = analyze_url(url_str)?;
    let hash = match fetcher_info.hash {
        Some(h) => h,
        None => calculate_hash(&fetcher_info).await?,
    };

    match fetcher_info.fetcher_type {
        FetcherType::GitHub => super::expression::generate_github_expression(&fetcher_info, &hash).await,
        FetcherType::URL => super::expression::generate_url_expression(&fetcher_info, &hash),
        FetcherType::Git => super::expression::generate_git_expression(&fetcher_info, &hash),
        FetcherType::GitLab => super::expression::generate_gitlab_expression(&fetcher_info, &hash).await,
        FetcherType::FromGitea => super::expression::generate_gitea_expression(&fetcher_info, &hash),
    }
}

pub fn analyze_url(url_str: &str) -> Result<FetcherInfo, Box<dyn std::error::Error>> {
    let url = Url::parse(url_str)?;
    
    let fetcher_type = match url.host_str() {
        Some("github.com") => FetcherType::GitHub,
        Some("gitlab.com") => FetcherType::GitLab,
        Some(host) if host.contains("gitea") => FetcherType::FromGitea,
        _ if url_str.ends_with(".git") => FetcherType::Git,
        _ => FetcherType::URL,
    };

    Ok(FetcherInfo {
        url: url_str.to_string(),
        fetcher_type,
        hash: None,
        version: extract_version(&url),
    })
}

pub async fn calculate_hash(info: &FetcherInfo) -> Result<String, Box<dyn std::error::Error>> {
    match info.fetcher_type {
        FetcherType::GitHub => {
            let parts: Vec<&str> = info.url.split('/').collect();
            if parts.len() >= 5 {
                let owner = parts[3];
                let repo = parts[4].trim_end_matches(".git");
                
                let client = reqwest::Client::new();
                let api_url = format!("https://api.github.com/repos/{}/{}", owner, repo);
                let response = client
                    .get(&api_url)
                    .header("User-Agent", "nix-prefetch")
                    .send()
                    .await?;
                
                let repo_info: GitHubRepo = response.json().await?;
                
                let output = Command::new("nix-prefetch-git")
                    .arg(&info.url)
                    .arg("--rev")
                    .arg(&repo_info.default_branch)
                    .output()?;
                
                let hash = String::from_utf8(output.stdout)?;
                Ok(hash.trim().to_string())
            } else {
                Err("Invalid GitHub URL".into())
            }
        }
        FetcherType::URL => {
            let output = Command::new("nix-prefetch-url")
                .arg(&info.url)
                .output()?;
            
            let hash = String::from_utf8(output.stdout)?;
            Ok(hash.trim().to_string())
        }
        _ => {
            let output = Command::new("nix-prefetch-git")
                .arg(&info.url)
                .output()?;
            
            let hash = String::from_utf8(output.stdout)?;
            Ok(hash.trim().to_string())
        }
    }
}

fn extract_version(url: &Url) -> Option<String> {
    let patterns = vec![
        Regex::new(r"v(\d+\.\d+\.\d+)").ok()?,
        Regex::new(r"releases/tag/(\d+\.\d+\.\d+)").ok()?,
        Regex::new(r"version[/-](\d+\.\d+\.\d+)").ok()?,
    ];

    for pattern in patterns {
        if let Some(captures) = pattern.captures(url.path()) {
            if let Some(version) = captures.get(1) {
                return Some(version.as_str().to_string());
            }
        }
    }

    None
}