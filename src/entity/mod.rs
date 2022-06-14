pub(crate) mod component;

use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Entity(pub usize);

impl Entity {
    pub fn as_u64(&self) -> u64 {
        self.0 as u64
    }
    pub fn index(&self) -> usize {
        self.0
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Entity#{:?}", self.as_u64()))
    }
}

impl From<usize> for Entity {
    fn from(e: usize) -> Self {
        Self(e)
    }
}
