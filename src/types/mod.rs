pub mod folder;
pub mod task;

use chrono::{DateTime, Local};
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Lifetime {
    creation_date: DateTime<Local>,
    lifetime_updates: Vec<DateTime<Local>>,
}
#[derive(Debug, Clone)]
pub enum Multiplier {
    Scalar(u32),
    Exponential{ frequency: u32 },
}