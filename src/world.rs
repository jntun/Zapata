use crate::error::MaxEntitiesError;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct World {
    name: String,
    max_entities: usize,
    entities: HashMap<String, Box<dyn Entity>>,
}

impl World {
    pub fn new(name: &str, max_entities: usize, team_count: usize, agent_count: usize) -> Self {
        Self {
            name: name.to_string(),
            max_entities,
            entities: HashMap::with_capacity(max_entities),
        }
    }
}

impl Debug for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.name.as_str())
            .field("entity count", &self.entities.len())
            .finish()
    }
}
