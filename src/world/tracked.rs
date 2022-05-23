use std::{
    fmt::{Debug, Display, Formatter},
    rc::Rc,
    cell::RefCell,
};
use crate::{
    entity,
    error::TickError,
    physics::PhysicsEntity,
    world::World,
};

pub enum Entity {
    Default(Box<dyn entity::Entity>),
    Physics(Box<dyn PhysicsEntity>),
}

impl entity::Entity for Entity {
    fn tick(&mut self, world: &World) -> Result<(), TickError> {
        match self {
            Entity::Default(e) => e.tick(world),
            Entity::Physics(e) => {
                let force = e.get_physx_data().get_effects_force_sum();
                e.mut_physx_data().apply_force(force);
                e.mut_physx_data().update_position();
                println!("{:?}\n", e.get_physx_data());
                e.tick(world)
            },
        }
    }

    fn get_name(&self) -> &str {
        match self {
            Entity::Default(e) => e.get_name(),
            Entity::Physics(e) => e.get_name(),
        }
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Default(e) => write!(f, "{:?}", e),
            Entity::Physics(e) => write!(f, "{:?}", e),
        }
    }
}