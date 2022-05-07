use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

pub trait Entity {
    fn tick(&mut self) -> Result<&mut Self, Error>
    where
        Self: Sized;

    fn get_display_name(&self) -> &str;
    fn get_id(&self) -> &str;
}

impl Debug for dyn Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.get_display_name()).finish()
    }
}
