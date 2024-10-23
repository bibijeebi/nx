pub mod config;
pub mod install;
pub mod remove;
pub mod gc;
pub mod optimize;
pub mod search;
pub mod update;
pub mod list;

// Re-export the command functions
pub use self::config::execute as config_execute;
pub use self::install::install;
pub use self::remove::execute as remove_execute;
pub use self::gc::execute as gc_execute;
pub use self::optimize::execute as optimize_execute;
pub use self::search::execute as search_execute;
pub use self::update::execute as update_execute;
pub use self::list::execute as list_execute;