use crate::World;
use crate::physics::{PhysicsEntity, PhysxData, vec3::Vec3};
use super::Entity;
use super::health::{Healther, Attacker, HealthStat, Health, Damage};
use super::TickError;
use std::fmt::Formatter;
use std::fmt::{Debug, Display};

const DEFAULT_HEALTH: u64 = 100;
const DEFAULT_MASS:   f64 = 20.0;
const DEFAULT_NAME:  &str = "default player";

#[derive(Clone)]
#[allow(dead_code)]
pub struct Human {
    name:   String,
    health: HealthStat,
    physx:  PhysxStats
}

impl Human {
    pub fn new(max_health: u64, start_health: Option<Health>, name: Option<String>, position: Option<Vec3>) -> Self {
        match name {
            Some(name) => Self {
                name,
                health: HealthStat::new(max_health, start_health),
                physx: PhysxStats::new(DEFAULT_MASS, position),
            },
            None => Self {
                name: String::from(DEFAULT_NAME),
                health: HealthStat::new(max_health, start_health),
                physx: PhysxStats::new(DEFAULT_MASS, position),
            }
        }
    }
}

impl PhysicsEntity for Human {
    fn apply_force(&mut self, force: Vec3) {
        self.physx.momentum *= force
    }

    fn get_mass(&self) -> f64 {
        self.physx.mass
    }

    fn get_momentum(&self) -> Vec3 {
        self.physx.momentum
    }

    fn get_pos(&self) -> Vec3 {
        self.physx.position
    }
}

impl Attacker for Human {
    fn attack<T: Healther>(&mut self, target: &mut T) {
        target.do_damage(self.get_damage());
        println!("{} just attacked '{}' for {} dmg", self, target.get_name(), self.get_damage());
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
    fn tick(&mut self, _world: &World) -> Result<(), TickError> {
        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl Display for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} - {}", self.name, self.get_current()))
    }
}

impl Debug for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.get_name())
            .field("health", &self.get_current())
            .field("max_health", &self.get_max())
            .field("physx", &self.physx)
            .finish()
    }
}


impl Default for Human {
    fn default() -> Self {
        Self {
            name: String::from(DEFAULT_NAME),
            health: HealthStat::new(DEFAULT_HEALTH, None),
            physx: PhysxStats::default(),
        }
    }
}
