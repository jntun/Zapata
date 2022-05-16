use std::fmt::{Debug, Formatter};
use std::cell::RefCell;
use std::rc::Rc;
use crate::entity::Entity;
use crate::error::TickError;

const DEFAULT_NAME: &str = "Zapata";

pub struct World {
    name: String,
    entities: Vec<Rc<RefCell<Box<dyn Entity>>>>,
}

impl World {
    pub fn new(name: Option<String>) -> Self {
        let entities = Vec::new();
        match name {
            Some(name) => Self {
                name,
                entities,
            },
            None => Self {
                name: String::from(DEFAULT_NAME),
                entities,
            }
        }
    }

    pub fn add_entity(&mut self, e: Rc<RefCell<Box<dyn Entity>>>) {
        self.entities.push(e.clone());
    }


}

impl Entity for World {
    fn tick(&mut self) -> Result<(), TickError> {
        for entity in self.entities.iter() {
            match entity.try_borrow_mut() {
                Ok(mut e) => {
                    match e.tick() {
                        Ok(()) => continue,
                        Err(e) => return Err(TickError::from(e)),
                    }
                },
                Err(e) => return Err(TickError::new(format!("world.tick(): failed to borrow entity - {}", e).as_str())),
            }
        }
        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl Debug for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.get_name())
            .field("entities", &self.entities)
            .finish()
    }
}