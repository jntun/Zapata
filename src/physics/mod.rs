pub(crate) mod vec3;
pub(crate) mod effect;

use {
    std::fmt::{Display, Debug, Formatter},
    crate::entity::Entity,
    self::vec3::Vec3,
    effect::Effect,
};

pub trait PhysicsEntity {
    fn get_physx_data(&self) -> &PhysxComponent;
    fn mut_physx_data(&mut self) -> &mut PhysxComponent;
}

#[derive(Clone)]
pub struct PhysxComponent {
    mass:     f64,
    momentum: Vec3,
    position: Vec3,
    effects:  Vec<Effect>,
}

impl PhysxComponent {
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

impl Default for PhysxComponent {
    fn default() -> Self {
        Self {
            mass: 1.0,
            momentum: Vec3::default(),
            position: Vec3::default(),
            effects: Vec::new(),
        }
    }
}

impl Debug for PhysxComponent {
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
        f.debug_struct("")
            .field("x", &self.get_physx_data().position.x())
            .field("y", &self.get_physx_data().position.y())
            .field("z", &self.get_physx_data().position.z())
            .field("effects", &self.get_physx_data().effects)
            .finish()
    }
}