pub mod config;
pub mod gc;
pub mod install;
pub mod list;
pub mod optimize;
pub mod remove;
pub mod search;
pub mod system;
pub mod update;

pub use self::config::execute as config_execute;
pub use self::gc::execute as gc_execute;
pub use self::install::install;
pub use self::list::execute as list_execute;
pub use self::optimize::execute as optimize_execute;
pub use self::remove::execute as remove_execute;
pub use self::search::execute as search_execute;
pub use self::system::execute as system_execute;
pub use self::update::execute as update_execute;
