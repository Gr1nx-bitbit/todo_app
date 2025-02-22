pub mod folder;
pub mod task;

use chrono::{DateTime, Local};
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Lifetime {
    creation_date: DateTime<Local>,
    lifetime_updates: Vec<DateTime<Local>>,
}

pub type FrequencyScalar = (u32, u32); // frequency in days as well as how much to exponentiate the task priority

#[derive(Debug, Clone)]
pub enum Multiplier {
    Scalar(u32), // how much to multiply a task's priority by
    Exponential(FrequencyScalar),
}