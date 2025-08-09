use crate::bot::base::Bot;

pub mod random;
pub mod basic;
pub mod adapt;
pub mod weak;

pub fn map_bot_string(name: &str) -> Option<Box<dyn Bot>> {
    if name == "random" { Some(Box::new(random::RandomBot::new())) } 
    else if let Some(num) = name.strip_prefix("weak") {
        if let Ok(n) = num.parse::<u32>() {
            if n == 0 { return None; }
            return Some(Box::new(weak::WeakBot::new(n)));
        }
        None
    }
    else if let Some(num) = name.strip_prefix("basic") {
        if let Ok(n) = num.parse::<u32>() {
            if n == 0 { return None; }
            return Some(Box::new(basic::BasicBot::new(n)));
        }
        None
    }
    else if let Some(num) = name.strip_prefix("adapt") {
        if let Ok(n) = num.parse::<u32>() {
            if n == 0 { return None; }
            return Some(Box::new(adapt::AdaptiveBot::new(2_u64.pow(n+4))));
        }
        None
    }
    else { None }
}