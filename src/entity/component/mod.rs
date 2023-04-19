pub(crate) mod collider;
pub(crate) mod health;
pub(crate) mod physics;
pub(crate) mod printer;

use crate::{
    entity::Entity,
    error::ZapataError,
    physics::{effect, hitbox, vec3::Vec3},
};

pub trait Component {
    fn update(&mut self, self_entity: Entity) -> Result<(), ZapataError>;

    fn name(&self) -> &str;
}

#[derive(Debug)]
pub struct Collider {
    pub hitbox: Vec<hitbox::Hitbox>,
}

pub struct Health {
    start: health::HealthUnit,
    current: health::HealthUnit,
    max: health::HealthUnit,
    damage_log: Vec<health::DamageEntry>,
}

#[derive(Clone)]
pub struct Physics {
    on: bool,
    mass: f64,
    momentum: Vec3,
    position: Vec3,
    effects: Vec<effect::Effect>,
}

pub struct Printer(bool);
