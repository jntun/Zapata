use crate::error;
use std::fmt::{Debug, Formatter};

pub trait Entity {
    fn tick(&mut self) -> Option<error::TickError>;

    fn get_id(&self) -> u64;
}

impl Debug for dyn Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Entity#{}", self.get_id()))
    }
}
