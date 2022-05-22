pub(crate) mod vec3;

use std::fmt::{Display, Debug, Formatter};
use crate::entity::Entity;
use self::vec3::Vec3;

pub(crate) trait PhysicsEntity: Entity {
    fn apply_force(&mut self, force: Vec3);
    fn get_mass(&self) -> f64;
    fn get_momentum(&self) -> Vec3;
    fn get_pos(&self) -> Vec3;
}

#[derive(Clone)]
pub struct PhysxData {
    mass:     f64,
    momentum: Vec3,
    position: Vec3,
}

impl PhysxData {
    pub fn new(mass: f64, position: Option<Vec3>) -> Self {
        match position {
            Some(position) => Self {
                mass,
                momentum: Vec3::new(0.0, 0.0, 0.0),
                position,
            },
            None => Self {
                mass,
                momentum: Vec3::new(0.0, 0.0, 0.0),
                position: Vec3::new(0.0, 0.0, 0.0),
            }
        }
    }
}

impl Default for PhysxData {
    fn default() -> Self {
        Self {
            mass: 0.0,
            momentum: Vec3::default(),
            position: Vec3::default(),
        }
    }
}

impl Debug for PhysxData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("data")
            .field("mass", &self.mass)
            .field("momentum", &self.momentum)
            .field("position", &self.position)
            .field("effects", &self.effects)
            .finish()
    }
}

impl Debug for dyn PhysicsEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.get_name())
            .field("x", &self.get_physx_data().position.x())
            .field("y", &self.get_physx_data().position.y())
            .field("z", &self.get_physx_data().position.z())
            .field("effects", &self.get_physx_data().effects)
            .finish()
    }
}