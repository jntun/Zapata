use crate::Entity;
use std::fmt::Error;

#[derive(Debug)]
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
