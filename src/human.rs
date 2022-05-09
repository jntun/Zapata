use crate::error;
use crate::Entity;
use std::fmt::{Debug, Display};
use std::fmt::{Error, Formatter};

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct Human {
    id: u64,
    current_health: u64,
    max_health: u64,
}

impl Human {
    pub fn new(id: u64, name: String, max_health: u64) -> Self {
        Self {
            id,
            current_health: max_health,
            max_health,
        }
    }
}

impl Entity for Human {
    fn tick(&mut self) -> Option<error::TickError> {
        println!("{} - ticked", self);

        None
    }

    fn get_id(&self) -> u64 {
        self.id
    }
}

impl Display for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("test {}", self.current_health))
    }
}

impl Debug for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("human").finish()
    }
}
