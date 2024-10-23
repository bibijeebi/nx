use crate::config::store::Config;

pub fn execute(allow_unfree: Option<bool>, config: &mut Config) -> i32 {
    if let Some(allow) = allow_unfree {
        config.allow_unfree = allow;
        config.save();
        println!("Updated unfree package setting: {}", if allow { "allowed" } else { "disallowed" });
    } else {
        println!("Current settings:");
        println!("  Allow unfree packages: {}", config.allow_unfree);
    }
    0
}