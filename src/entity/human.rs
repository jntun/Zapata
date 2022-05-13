use super::health::{HealthTracker, Health};
use super::Entity;
use super::TickError;
use std::fmt::Formatter;
use std::fmt::{Debug, Display};

const DEFAULT_HEALTH: u64 = 100;
const DEFAULT_NAME: &str = "default player";

#[allow(dead_code)]
pub struct Human {
    name: String,
    health: HealthTracker,
}

impl Human {
    pub fn new(max_health: u64, start_health: Option<u64>, name: Option<String>) -> Self {
        match name {
            Some(name) => Self {
                name,
                health: HealthTracker::new(max_health, start_health),
            },
            None => Self {
                name: String::from(DEFAULT_NAME),
                health: HealthTracker::new(max_health, start_health),
            }
        }
    }
}

impl Health for Human {
    fn do_damage(&mut self, dmg: u64) {
        self.health.do_damage(dmg)
    }

    fn get_current(&self) -> u64 {
        self.health.get_current()
    }

    fn get_max(&self) -> u64 {
        self.health.get_max()
    }
}

impl Entity for Human {
    fn tick(&mut self) -> Option<TickError> {
        println!("{} - ticked", self);

        None
    }
}

impl Display for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} - {}", self.name, self.health.get_current()))
    }
}

impl Debug for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("human")
            .field("health", &self.health.get_current())
            .field("max_health", &self.health.get_max())
            .finish()
    }
}


impl Default for Human {
    fn default() -> Self {
        Self {
            name: String::from(DEFAULT_NAME),
            health: HealthTracker::new(DEFAULT_HEALTH, None),
        }
    }
}
