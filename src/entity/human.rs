use super::Entity;
use super::health::{Healther, Attacker, HealthStat, Health, Damage};
use super::TickError;
use std::fmt::Formatter;
use std::fmt::{Debug, Display};

const DEFAULT_HEALTH: u64 = 100;
const DEFAULT_NAME: &str = "default player";

#[allow(dead_code)]
pub struct Human {
    name: String,
    health: HealthStat,
}

impl Human {
    pub fn new(max_health: u64, start_health: Option<Health>, name: Option<String>) -> Self {
        match name {
            Some(name) => Self {
                name,
                health: HealthStat::new(max_health, start_health),
            },
            None => Self {
                name: String::from(DEFAULT_NAME),
                health: HealthStat::new(max_health, start_health),
            }
        }
    }
}

impl Attacker for Human {
    fn attack<T: Healther>(&mut self, target: &mut T) {
        let dmg = self.get_damage();
        target.do_damage(dmg);
    }

    fn get_damage(&self) -> Damage {
        1 as u64
    }
}

impl Healther for Human {
    fn do_damage(&mut self, dmg: Damage) {
        self.health -= dmg;
    }

    fn set_max(&mut self, health: Health) {
        self.health.max = health;
    }

    fn get_current(&self) -> Health {
        self.health.current
    }

    fn get_max(&self) -> Health {
        self.health.max
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
        f.write_fmt(format_args!("{} - {}", self.name, self.get_current()))
    }
}

impl Debug for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("human")
            .field("health", &self.get_current())
            .field("max_health", &self.get_max())
            .finish()
    }
}


impl Default for Human {
    fn default() -> Self {
        Self {
            name: String::from(DEFAULT_NAME),
            health: HealthStat::new(DEFAULT_HEALTH, None),
        }
    }
}
