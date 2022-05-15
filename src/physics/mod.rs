pub(crate) mod vec3;

use crate::entity::Entity;
use self::vec3::Vec3;

pub(crate) trait PhysicsEntity: Entity {
    fn apply_force(&mut self, force: Vec3);
    fn get_mass(&self) -> f64;
    fn get_momentum(&self) -> Vec3;
    fn get_pos(&self) -> Vec3;
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct PhysxStats {
    pub momentum: Vec3,
    pub position: Vec3,
    pub mass:     f64,
}

impl PhysxStats {
    pub fn new(mass: f64, position: Option<Vec3>) -> Self {
        match position {
            Some(pos) => Self {
                momentum: Vec3::new(0.0, 0.0, 0.0),
                position: pos,
                mass,
            },
            None => Self {
                momentum: Vec3::new(0.0, 0.0, 0.0),
                position: Vec3::new(0.0, 0.0, 0.0),
                mass,
            }
        }
    }
}

impl Default for PhysxStats {
    fn default() -> Self {
        Self {
            momentum: Vec3::default(),
            position: Vec3::default(),
            mass: 0.0,
        }
    }
}