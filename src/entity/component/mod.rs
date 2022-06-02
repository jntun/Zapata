pub(crate) mod collider;
pub(crate) mod health;
pub(crate) mod physics;
pub(crate) mod printer;

use std::fmt::Formatter;
use {
    crate::{entity::Entity, error::ZapataError, scene::Scene},
    std::fmt::Debug,
};

pub trait Component {
    fn update(&mut self, entity: Entity, scene: &Scene) -> Result<(), ZapataError>;
    fn is_active(&self) -> bool;
    fn get_name(&self) -> &str;
}

impl Debug for dyn Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.get_name())
            .field("active", &self.is_active())
            .finish()
    }
}
