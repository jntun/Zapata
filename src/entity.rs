pub(crate) mod human;
pub(crate) mod health;

use crate::World;
use crate::error::TickError;
use std::fmt::{Debug, Formatter};


pub trait Entity {
    fn tick(&mut self, world: &World) -> Result<(), TickError>;
    fn get_name(&self) -> &str;
}

impl Debug for dyn Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Entity"))
    }
}