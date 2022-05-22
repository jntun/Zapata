use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::entity::Entity;
use crate::error::TickError;
use crate::physics::{
    PhysicsEntity,
    vec3::Vec3,
};
use crate::world::World;

#[derive(Debug, Clone)]
pub struct Effect {
    name: String,
    force: Vec3,
    duration: Duration,
}

impl Effect {
    pub fn apply_force(&self, mut target: impl PhysicsEntity) {
        target.mut_physx_data().momentum+=self.force;
    }

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