pub(crate) mod vec3;
pub(crate) mod effect;

use std::fmt::{Display, Debug, Formatter};
use crate::entity::Entity;
use self::vec3::Vec3;
use effect::Effect;

pub trait PhysicsEntity: Entity {
    fn get_physx_data(&self) -> &PhysxData;
    fn mut_physx_data(&mut self) -> &mut PhysxData;
}

#[derive(Clone)]
pub struct PhysxData {
    mass:     f64,
    momentum: Vec3,
    position: Vec3,
    effects:  Vec<Effect>,
}

impl PhysxData {
    pub fn new(mass: f64, position: Option<Vec3>) -> Self {
        match position {
            Some(position) => Self {
                mass,
                momentum: Vec3::new(0.0, 0.0, 0.0),
                position,
                effects: Vec::new(),
            },
            None => Self {
                mass,
                momentum: Vec3::new(0.0, 0.0, 0.0),
                position: Vec3::new(0.0, 0.0, 0.0),
                effects: Vec::new(),
            }
        }
    }

    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn get_effects_force_sum(&self) -> Vec3 {
        let mut total = Vec3::default();
        for effect in self.effects.iter() {
            total += effect.get_force();
        }
        total
    }

    pub fn apply_force(&mut self, force: Vec3) {
        self.momentum += force
    }
    pub fn update_position(&mut self) {
        self.position += self.momentum / self.mass
    }
}

impl Default for PhysxData {
    fn default() -> Self {
        Self {
            mass: 0.0,
            momentum: Vec3::default(),
            position: Vec3::default(),
            effects: Vec::new(),
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