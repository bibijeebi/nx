use super::fetcher::FetcherInfo;

pub async fn generate_github_expression(
    info: &FetcherInfo,
    hash: &str,
) -> Result<String, Box<dyn std::error::Error>> {
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

pub async fn generate_gitlab_expression(
    info: &FetcherInfo,
    hash: &str,
) -> Result<String, Box<dyn std::error::Error>> {
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

pub fn generate_url_expression(
    info: &FetcherInfo,
    hash: &str,
) -> Result<String, Box<dyn std::error::Error>> {
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

pub fn generate_git_expression(
    info: &FetcherInfo,
    hash: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let name = info
        .url
        .split('/')
        .last()
        .unwrap_or("source")
        .trim_end_matches(".git");

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

pub fn generate_gitea_expression(
    info: &FetcherInfo,
    hash: &str,
) -> Result<String, Box<dyn std::error::Error>> {
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
