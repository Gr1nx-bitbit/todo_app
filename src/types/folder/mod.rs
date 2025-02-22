use chrono::{DateTime, Local};
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Lifetime {
    creation_date: DateTime<Local>,
    lifetime_updates: Vec<DateTime<Local>>,
}