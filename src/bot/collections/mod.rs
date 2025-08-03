use crate::bot::base::Bot;

pub mod random;
pub mod basic;

pub fn map_bot_string(name: &str) -> Option<Box<dyn Bot>> {
    if name == "random" {
        Some(Box::new(random::RandomBot))
    } 
    else if name == "basic" {
        Some(Box::new(basic::BasicBot))
    }
    else {
        None
    }
}