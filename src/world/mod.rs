pub(crate) mod tracked;

use std::fmt::{Debug, Formatter};
use std::cell::RefCell;
use std::rc::Rc;
use crate::entity::Entity;
use crate::error::TickError;
use crate::physics::effect::Effect;
use crate::physics::vec3::Vec3;

const DEFAULT_NAME: &str = "Zapata";

pub struct World {
    name: String,
    entities: Vec<Rc<RefCell<tracked::Entity>>>,
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

    pub fn add_entity(&mut self, e: tracked::Entity) {
        match e {
            tracked::Entity::Default(entity) => self.entities.push(Rc::new(RefCell::new(tracked::Entity::Default(entity)))),
            tracked::Entity::Physics(mut physx_entity) => {
                physx_entity.mut_physx_data().add_effect(self.get_gravity());
                self.entities.push(Rc::new(RefCell::new(tracked::Entity::Physics(physx_entity))));
            }
        };
    }

    pub fn get_gravity(&self) -> Effect {
        Effect::new(String::from("Gravity"), Vec3::new(0.0, 9.821, 0.0), None)
    }
}

impl World {
    fn pre_tick(&mut self) -> Result<(), TickError> {
        Ok(())
    }

    fn tick_entities(&mut self) -> Result<(), TickError> {
        for entity in self.entities.iter() {
            match entity.try_borrow_mut() {
                Ok(mut e) => {
                    match e.tick(self) {
                        Ok(()) => continue,
                        Err(e) => return Err(TickError::from(e)),
                    }
                },
                Err(e) => return Err(TickError::new(format!("world.tick(): failed to borrow entity - {}", e).as_str())),
            }
        }
        Ok(())
    }

    fn post_tick(&mut self) -> Result<(), TickError> {
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), TickError> {
        if let Err(e) = self.tick_entities() {
            return Err(e);
        } else {
            Ok(())
        }
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