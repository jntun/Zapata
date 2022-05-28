pub(crate) mod health;
pub(crate) mod physics;

use crate::{
    error::ZapataError,
    entity::Entity,
    scene::Scene,
};

pub trait Component {
    fn update(&mut self, entity: Entity, scene: &Scene) -> Result<(), ZapataError>;
    fn is_active(&self) -> bool;
    fn get_name(&self) -> &str;
}