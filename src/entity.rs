pub(crate) mod human;
pub(crate) mod health;

use crate::error::TickError;
use std::fmt::{Debug, Formatter};

pub trait Entity {
    fn tick(&mut self) -> Option<TickError>;
}

impl Debug for dyn Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Entity"))
    }
}
