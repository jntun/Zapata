pub(crate) mod collider;
pub(crate) mod health;
pub(crate) mod physics;
pub(crate) mod printer;

use crate::{entity::Entity, error::ZapataError};

pub trait Component {
    fn update(&mut self, self_entity: Entity) -> Result<(), ZapataError>;

    fn name(&self) -> &str;
}
