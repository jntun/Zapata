use crate::error;
use std::fmt::{Debug, Formatter};

pub trait Entity {
    fn tick(&mut self) -> Option<error::TickError>;

    fn get_display_name(&self) -> &str;
    fn get_id(&self) -> &str;
}

impl Debug for dyn Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.get_display_name()).finish()
    }
}
