use super::fetcher::{FetcherInfo};

use reqwest;
use sha2::{Sha256, Digest};
use tokio;
use url::Url;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct GitHubRepo {
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
    url: String,
    fetcher_type: FetcherType,
    hash: Option<String>,
    version: Option<String>,
}

pub async fn generate_expression(url_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let fetcher_info = analyze_url(url_str)?;
    let hash = match fetcher_info.hash {
        Some(h) => h,
        None => calculate_hash(&fetcher_info).await?,
    };

    match fetcher_info.fetcher_type {
        FetcherType::GitHub => generate_github_expression(&fetcher_info, &hash).await,
        FetcherType::URL => generate_url_expression(&fetcher_info, &hash),
        FetcherType::Git => generate_git_expression(&fetcher_info, &hash),
        FetcherType::GitLab => generate_gitlab_expression(&fetcher_info, &hash).await,
        FetcherType::FromGitea => generate_gitea_expression(&fetcher_info, &hash),
    }
}

fn analyze_url(url_str: &str) -> Result<FetcherInfo, Box<dyn std::error::Error>> {
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

async fn calculate_hash(info: &FetcherInfo) -> Result<String, Box<dyn std::error::Error>> {
    match info.fetcher_type {
        FetcherType::GitHub => {
            let parts: Vec<&str> = info.url.split('/').collect();
            if parts.len() >= 5 {
                let owner = parts[3];
                let repo = parts[4].trim_end_matches(".git");
                
                // Use GitHub API to get repo info
                let client = reqwest::Client::new();
                let api_url = format!("https://api.github.com/repos/{}/{}", owner, repo);
                let response = client
                    .get(&api_url)
                    .header("User-Agent", "nix-prefetch")
                    .send()
                    .await?;
                
                let repo_info: GitHubRepo = response.json().await?;
                
                // Use nix-prefetch-git to get the hash
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
            // Use nix-prefetch-url
            let output = Command::new("nix-prefetch-url")
                .arg(&info.url)
                .output()?;
            
            let hash = String::from_utf8(output.stdout)?;
            Ok(hash.trim().to_string())
        }
        _ => {
            // Default to using nix-prefetch-git for other git sources
            let output = Command::new("nix-prefetch-git")
                .arg(&info.url)
                .output()?;
            
            let hash = String::from_utf8(output.stdout)?;
            Ok(hash.trim().to_string())
        }
    }
}

async fn generate_github_expression(info: &FetcherInfo, hash: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = info.url.split('/').collect();
    let (owner, repo) = (parts[3], parts[4].trim_end_matches(".git"));
    
    let version = info.version.as_deref().unwrap_or("HEAD");
    
    Ok(format!(
        r#"{{ pkgs ? import <nixpkgs> {{}} }}:

pkgs.stdenv.mkDerivation rec {{
  pname = "{}";
  version = "{}";

  src = pkgs.fetchFromGitHub {{
    owner = "{}";
    repo = "{}";
    rev = version;
    sha256 = "{}";
  }};

  buildInputs = with pkgs; [
    # Add your dependencies here
  ];

  meta = with pkgs.lib; {{
    description = "";
    homepage = "{}";
    license = licenses.mit;  # Adjust license accordingly
    maintainers = with maintainers; [ ];
    platforms = platforms.all;
  }};
}}
"#,
        repo, version, owner, repo, hash, info.url
    ))
}

fn generate_url_expression(info: &FetcherInfo, hash: &str) -> Result<String, Box<dyn std::error::Error>> {
    let filename = info.url.split('/').last().unwrap_or("source");
    
    Ok(format!(
        r#"{{ pkgs ? import <nixpkgs> {{}} }}:

pkgs.stdenv.mkDerivation rec {{
  pname = "{}";
  version = "{}";

  src = pkgs.fetchurl {{
    url = "{}";
    sha256 = "{}";
  }};

  buildInputs = with pkgs; [
    # Add your dependencies here
  ];

  meta = with pkgs.lib; {{
    description = "";
    homepage = "{}";
    license = licenses.mit;  # Adjust license accordingly
    maintainers = with maintainers; [ ];
    platforms = platforms.all;
  }};
}}
"#,
        filename.split('.').next().unwrap_or("package"),
        info.version.as_deref().unwrap_or("1.0.0"),
        info.url,
        hash,
        info.url
    ))
}

fn generate_git_expression(info: &FetcherInfo, hash: &str) -> Result<String, Box<dyn std::error::Error>> {
    let name = info.url.split('/').last().unwrap_or("source").trim_end_matches(".git");
    
    Ok(format!(
        r#"{{ pkgs ? import <nixpkgs> {{}} }}:

pkgs.stdenv.mkDerivation rec {{
  pname = "{}";
  version = "{}";

  src = pkgs.fetchgit {{
    url = "{}";
    sha256 = "{}";
    rev = "HEAD";  # Adjust revision accordingly
  }};

  buildInputs = with pkgs; [
    # Add your dependencies here
  ];

  meta = with pkgs.lib; {{
    description = "";
    homepage = "{}";
    license = licenses.mit;  # Adjust license accordingly
    maintainers = with maintainers; [ ];
    platforms = platforms.all;
  }};
}}
"#,
        name,
        info.version.as_deref().unwrap_or("1.0.0"),
        info.url,
        hash,
        info.url
    ))
}

async fn generate_gitlab_expression(info: &FetcherInfo, hash: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = info.url.split('/').collect();
    let (owner, repo) = (parts[3], parts[4].trim_end_matches(".git"));
    
    Ok(format!(
        r#"{{ pkgs ? import <nixpkgs> {{}} }}:

pkgs.stdenv.mkDerivation rec {{
  pname = "{}";
  version = "{}";

  src = pkgs.fetchFromGitLab {{
    owner = "{}";
    repo = "{}";
    rev = version;
    sha256 = "{}";
  }};

  buildInputs = with pkgs; [
    # Add your dependencies here
  ];

  meta = with pkgs.lib; {{
    description = "";
    homepage = "{}";
    license = licenses.mit;  # Adjust license accordingly
    maintainers = with maintainers; [ ];
    platforms = platforms.all;
  }};
}}
"#,
        repo,
        info.version.as_deref().unwrap_or("1.0.0"),
        owner,
        repo,
        hash,
        info.url
    ))
}

fn generate_gitea_expression(info: &FetcherInfo, hash: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = info.url.split('/').collect();
    let (owner, repo) = (parts[3], parts[4].trim_end_matches(".git"));
    
    Ok(format!(
        r#"{{ pkgs ? import <nixpkgs> {{}} }}:

pkgs.stdenv.mkDerivation rec {{
  pname = "{}";
  version = "{}";

  src = pkgs.fetchFromGitea {{
    domain = "{}";
    owner = "{}";
    repo = "{}";
    rev = version;
    sha256 = "{}";
  }};

  buildInputs = with pkgs; [
    # Add your dependencies here
  ];

  meta = with pkgs.lib; {{
    description = "";
    homepage = "{}";
    license = licenses.mit;  # Adjust license accordingly
    maintainers = with maintainers; [ ];
    platforms = platforms.all;
  }};
}}
"#,
        repo,
        info.version.as_deref().unwrap_or("1.0.0"),
        parts[2],
        owner,
        repo,
        hash,
        info.url
    ))
}

fn extract_version(url: &Url) -> Option<String> {
    // Try to extract version from common patterns in URLs
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