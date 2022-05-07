use crate::Entity;
use std::fmt::{Debug, Display};
use std::fmt::{Error, Formatter};

#[derive(Clone)]
#[allow(dead_code)]
pub struct Human {
    name: String,
    current_health: u64,
    max_health: u64,
}

impl Human {
    pub fn new(name: &str, max_health: u64) -> Self {
        Self {
            name: name.to_string(),
            current_health: max_health,
            max_health,
        }
    }
}

impl Entity for Human {
    fn tick(&mut self) -> Result<&mut Self, Error> {
        println!("{}", self.name);

        Ok(self)
    }

    fn get_display_name(&self) -> &str {
        &self.name
    }

    fn get_id(&self) -> &str {
        // TODO: make real
        &self.name
    }
}

impl Display for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.get_display_name(), self.current_health)
    }
}

impl Debug for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.get_display_name(), self.current_health)
    }
}
