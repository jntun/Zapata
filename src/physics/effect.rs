use {
    std::fmt::{Debug, Formatter},
    crate::{
        entity::Entity,
        error::TickError,
        physics::{
            vec3::Vec3,
        },
    }
};

use crate::world::World;

#[derive(Debug, Clone)]
pub struct Effect {
    name: String,
    force: Vec3,
    duration: Duration,
}

impl Effect {
    pub fn get_force(&self) -> Vec3 { self.force }

    pub fn is_alive(&self, current_tick: u64) -> bool {
        match self.duration {
            Duration::Indefinite => true,
            Duration::EndTick(end) => current_tick >= end,
        }
    }

    pub fn new(name: String, force: Vec3, duration: Option<Duration>) -> Self {
        if let Some(duration) = duration {
            Self {
                name,
                force,
                duration,
            }
        } else {
            Self {
                name,
                force,
                duration: Duration::Indefinite,
            }
        }
    }
}

impl Entity for Effect {
    fn tick(&mut self, world: &World) -> Result<(), TickError> {
        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Duration {
    Indefinite,
    EndTick(u64)
}