use crate::Entity;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

pub struct World<'a> {
    name: String,
    entities: HashMap<String, Box<&'a dyn Entity>>,
}

impl<'a> World<'a> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entities: HashMap::new(),
        }
    }

    pub fn add_entity(&'a mut self, e: Box<&'a dyn Entity>) -> Result<&'a mut Self, Error> {
        self.entities.insert(e.get_id().to_string(), e);
        Ok(self)
    }
}

impl Entity for World<'_> {
    fn tick(&mut self) -> Result<&mut Self, Error>
    where
        Self: Sized,
    {
        todo!()
    }
    fn get_display_name(&self) -> &str {
        &self.name
    }
    fn get_id(&self) -> &str {
        todo!()
    }
}

impl Debug for World<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.get_display_name())
            .field("entity count", &self.entities.len())
            .finish()
    }
}
