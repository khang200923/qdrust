use crate::bot::base::Bot;

pub mod random;
pub mod basic;

pub fn map_bot_string(name: &str) -> Option<Box<dyn Bot>> {
    if name == "random" { Some(Box::new(random::RandomBot)) } 
    else if let Some(num) = name.strip_prefix("basic") {
        if let Ok(n) = num.parse::<u32>() {
            if n == 0 { return None; }
            return Some(Box::new(basic::BasicBot::new(n)));
        }
        None
    }
    else { None }
}