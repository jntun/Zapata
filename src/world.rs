use crate::Entity;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::ops::Index;

pub struct World<'a> {
    name: String,
    max_entities: usize,
    entities: HashMap<String, &'a Box<dyn Entity>>,
}

impl<'a> World<'a> {
    pub fn new(name: &str, max_entities: usize, team_count: usize, agent_count: usize) -> Self {
        Self {
            name: name.to_string(),
            max_entities,
            entities: HashMap::with_capacity(max_entities),
        }
    }

    pub fn add_entity(&'a mut self, e: &'a Vec<Box<dyn Entity>>) -> Result<&'a mut Self, Error> {
        if self.entities.len() == self.max_entities {
            panic!("max entities reached")
        }
        //self.entities.insert(e.get_id().to_string(), e);
        for i in 0..e.len() {
            let entity = e.index(i);
            self.entities
                .insert(entity.get_id().to_string(), entity.clone());
        }
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
