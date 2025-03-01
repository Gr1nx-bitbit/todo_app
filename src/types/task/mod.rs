use chrono::{DateTime, Local};
use crate::values;

#[derive(Debug, Clone)]
pub struct Task {
    objective: String,
    priority_level: super::Priorities,
    priority_count: u32,
    multiplier: super::Multiplier,
    life: super::Lifetime,
}

impl Task {
    pub fn new_default() -> Task {
        Task {
            objective: String::from("Say Hello!"),
            priority_level: super::Priorities::Low,
            priority_count: values::BASE_PRIORITY_POINTS,
            multiplier: super::Multiplier::Scalar(values::BASE_SCALAR),
            life: super::Lifetime {
                creation_date: Local::now(),
                lifetime_updates: Vec::<DateTime<Local>>::new(),
            },
        }
    }

    pub fn new_with_args(
        objective: String,
        priority_level: super::Priorities,
        priority_count: u32,
        multiplier: super::Multiplier ) -> Task {
        Task {
            objective,
            priority_level,
            priority_count,
            multiplier,
            life: super::Lifetime {
                creation_date: Local::now(),
                lifetime_updates: Vec::<DateTime<Local>>::new(),
            },
        }
    }

    pub fn update_priority(&mut self) {
        self.priority_count = match self.priority_level {
            super::Priorities::Low => self.priority_count + values::BASE_ADDITIVE_POINTS,
            super::Priorities::Medium => self.priority_count + values::BASE_ADDITIVE_POINTS,
            super::Priorities::High => self.priority_count + values::BASE_ADDITIVE_POINTS,
            super::Priorities::Exponent((base, exponent)) => base.pow(exponent), // need to add some logic to make sure it doesn't go over max
            super::Priorities::Max => values::MAX_PRIORITY_COUNT,
        };

        self.enforce_priority_count();
    }

    fn enforce_priority_count(&mut self) { // enforces priority count is under MAX_PRIORITY_COUNT
        if self.priority_count > values::MAX_PRIORITY_COUNT {
            self.priority_count = values::MAX_PRIORITY_COUNT;
            if !self.is_max_priroity() {
                self.priority_level = super::Priorities::Max;
            }
        } else if self.priority_count == values::MAX_PRIORITY_COUNT {
            if !self.is_max_priroity() {
                self.priority_level = super::Priorities::Max;
            }
        }
    }

    fn is_max_priroity(&self) -> bool {
        match self.priority_level {
            super::Priorities::Max => true,
            _ => false,
        }
    }

    pub fn print(&self) {
        let task = format!("Objective: {} \nPriority Level: {:?} \nPriority Count: {} \nMultiplier: {:?} \nCreation Date: {:?} \nLifetime Updates: {:?}",
        self.objective, self.priority_level, self.priority_count, self.multiplier, self.life.creation_date.date_naive(), self.life.lifetime_updates);
        println!("{}", task);
    }

}