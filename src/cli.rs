use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nx")]
#[command(about = "A friendly wrapper for common Nix commands", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Additional arguments to pass to the underlying command
    #[arg(trailing_var_arg = true)]
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

    /// Modify NixOS system configuration
    #[command(alias = "sys")]
    System {
        #[command(subcommand)]
        command: SystemCommands,
    },
}

#[derive(Subcommand)]
pub enum SystemCommands {
    /// Add a package to system packages
    #[command(alias = "pkg")]
    Package {
        /// Package to add
        package: String,

        /// Make the change permanent (modifies configuration.nix)
        #[arg(short, long)]
        permanent: bool,
    },

    /// Enable a program or service
    #[command(alias = "en")]
    Enable {
        /// Program or service to enable (e.g., "programs.fish" or "services.docker")
        program: String,

        /// Make the change permanent (modifies configuration.nix)
        #[arg(short, long)]
        permanent: bool,
    },

    /// Set a NixOS option
    #[command(alias = "set")]
    SetOption {
        /// Option path (e.g., "programs.fish.enable" or "networking.hostName")
        path: String,

        /// Value to set (will be parsed as Nix expression)
        value: String,

        /// Make the change permanent (modifies configuration.nix)
        #[arg(short, long)]
        permanent: bool,
    },

    /// Apply temporary changes
    Apply,

    /// Show pending changes
    Show,
}
