pub(crate) mod component;

use {
    std::{
        fmt::{Debug, Formatter},
        hash::Hash,
    },
    crate:: error::ZapataError,
};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Entity(u64);

impl Entity {
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Entity#{:?}", self.as_u64()))
    }
}