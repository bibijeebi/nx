mod cli;
mod commands;
mod config;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let mut config = config::store::Config::load();

    let exit_code = match cli.command {
        Commands::Install { package } => {
            commands::install(&package, &cli.passthrough_args, &config)
        }
        Commands::Config { allow_unfree } => commands::config_execute(allow_unfree, &mut config),
        Commands::GarbageCollect {
            delete_old,
            foreground,
        } => commands::gc_execute(delete_old, foreground, &cli.passthrough_args, &config),
        Commands::Optimize { foreground } => {
            commands::optimize_execute(foreground, &cli.passthrough_args, &config)
        }
        Commands::Remove { all, package } => {
            commands::remove_execute(all, package, &cli.passthrough_args, &config)
        }
        Commands::Update { all, package } => {
            commands::update_execute(all, package, &cli.passthrough_args, &config)
        }
        Commands::Search { query } => {
            commands::search_execute(&query, &cli.passthrough_args, &config)
        }
        Commands::List => commands::list_execute(&cli.passthrough_args, &config),
        Commands::System { command } => {
            commands::system_execute(command, &cli.passthrough_args, &config)
        }
    };

    std::process::exit(exit_code);
}
