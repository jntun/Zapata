mod physics;

use crate::{
    entity::{component, ecs::ECS},
    error::ZapataError,
    physics::Effect,
    scene::Scene,
};

pub struct RegistryBuilder {
    reg: Registry,
}

impl RegistryBuilder {
    fn start(reg: Registry) -> Self {
        Self { reg }
    }

    pub fn enable_physics(mut self) -> Self {
        self.reg.physics = Some(Physics());
        self
    }

    pub fn done(self) -> Registry {
        self.reg
    }
}

#[derive(Debug)]
pub struct Registry {
    physics: Option<Physics>,
}

impl Registry {
    pub fn run_systems(&mut self, scene: &mut Scene) -> Result<(), ZapataError> {
        if let Some(physics) = self.physics.as_mut() {
            if let Err(e) = physics.run_system(scene) {
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn new() -> RegistryBuilder {
        RegistryBuilder::start(Self { physics: None })
    }
}

pub trait System {
    fn run_system(&mut self, scene: &mut Scene) -> Result<(), ZapataError>;
}

#[derive(Debug)]
pub struct Physics();
