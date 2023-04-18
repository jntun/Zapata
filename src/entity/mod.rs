pub(crate) mod component;
pub(crate) mod ecs;

use std::fmt::{Debug, Display, Formatter};

pub type Index = usize;
pub type Generation = u64;

#[derive(Copy, Clone)]
pub struct Entity {
    pub index: Index,
    pub generation: Generation,
}

impl Entity {
    pub fn index(&self) -> usize {
        self.index
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Entity#{:?}-g{}", self.index, self.generation))
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.index))
    }
}
