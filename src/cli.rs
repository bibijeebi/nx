use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nx")]
#[command(about = "A friendly wrapper for common Nix commands", long_about = None)]
#[command(trailing_var_arg = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Additional arguments to pass to the underlying command
    #[arg(last = true)]
    pub passthrough_args: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install a package (shorthand for 'nix profile install')
    #[command(alias = "i")]
    Install {
        /// Package name to install
        package: String,
    },
    
    /// Remove packages
    #[command(alias = "rm")]
    Remove {
        /// Remove all packages
        #[arg(short, long)]
        all: bool,
        
        /// Specific package to remove
        package: Option<String>,
    },
    
    /// Garbage collection (nix-collect-garbage)
    #[command(alias = "gc")]
    GarbageCollect {
        /// Delete old generations (-d flag)
        #[arg(short, long)]
        delete_old: bool,
        
        /// Run in foreground
        #[arg(short = 'f', long)]
        foreground: bool,
    },
    
    /// Optimize store (nix store optimize-store)
    #[command(alias = "o")]
    Optimize {
        /// Run in foreground
        #[arg(short = 'f', long)]
        foreground: bool,
    },
    
    /// List installed packages (nix profile list)
    #[command(alias = "ls")]
    List,
    
    /// Update packages (nix profile upgrade)
    #[command(alias = "u")]
    Update {
        /// Update all packages
        #[arg(short, long)]
        all: bool,
        
        /// Specific package to update
        package: Option<String>,
    },
    
    /// Search packages (nix search)
    #[command(alias = "s")]
    Search {
        /// Query string
        query: String,
    },

    /// Configure settings
    #[command(alias = "c")]
    Config {
        /// Allow unfree packages
        #[arg(long)]
        allow_unfree: Option<bool>,
    },
}