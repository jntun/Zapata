use crate::error;
use crate::error::MaxEntitiesError;
use crate::Entity;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::ops::Index;

pub struct World {
    name: String,
    max_entities: usize,
    entities: HashMap<String, Box<dyn Entity>>,
}

impl World {
    pub fn add_entity<T: 'static + Entity + Copy>(
        &mut self,
        entities: Vec<T>,
    ) -> Result<&mut Self, MaxEntitiesError> {
        for e in entities.iter() {
            if self.entities.len() == self.max_entities {
                return Err(MaxEntitiesError::new(
                    format!("max entities reached - {}", self.max_entities).as_str(),
                ));
            }
            self.entities
                .insert(e.get_id().to_string(), Box::new(e.clone()));
        }
        Ok(self)
    }

    pub fn new(name: &str, max_entities: usize, team_count: usize, agent_count: usize) -> Self {
        Self {
            name: name.to_string(),
            max_entities,
            entities: HashMap::with_capacity(max_entities),
        }
    }
}

impl Entity for World {
    fn tick(&mut self) -> Option<error::TickError> {
        for (k, e) in self.entities.iter_mut() {
            match e.tick() {
                Some(e) => return Some(e),
                None => continue,
            }
        }

        None
    }

    fn get_id(&self) -> u64 {
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
