pub(crate) mod component;

use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Entity(u64);

impl Entity {
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn new(id: u64) -> Self {
        Self(id)
    }


}

impl From<usize> for Entity {
    fn from(i: usize) -> Self {
        return Self(i as u64)
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Entity#{:?}", self.as_u64()))
    }
}