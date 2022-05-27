use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    cell::RefCell,
    rc::Rc,
    time,
};
use crate::{
    entity::{
        Component,
        Entity,
    },
    error::{
        ZapataError,
    },
    physics::{
        effect::Effect,
        vec3::Vec3,
    },
};

const DEFAULT_NAME: &str = "Zapata";

pub struct Scene {
    name:                  String,
    total_tick_time:       time::Duration,
    total_delta_tick_time: time::Duration,
    ticks:                 u64,
    ids:                   u64,
    entities:              HashMap<Entity, Vec<Rc<RefCell<Box<dyn Component>>>>>,
}


impl Scene {
    pub fn add_entity(&mut self, components: Vec<Box<dyn Component>>) -> Result<Entity, ZapataError> {
        let entity = self.new_entity();
        let comp_list = components.into_iter().map(|c| Rc::new(RefCell::new(c))).collect();
        self.entities.insert(entity.clone(), comp_list);
        Ok(entity)
    }

    fn new_entity(&mut self) -> Entity {
        let e = self.ids;
        self.ids += 1;
        Entity::new(e)
    }

    fn pre_update(&mut self) -> Result<(), ZapataError> {
        Ok(())
    }

    fn update_entities(&mut self) -> Result<(), ZapataError> {
        for (entity, comp_list) in self.entities.iter() {
            for comp in comp_list.iter() {
                match comp.try_borrow_mut() {
                    Ok(mut comp) => {
                        match comp.update(entity.clone(), self) {
                            Ok(()) => (),
                            Err(e) => return Err(e),
                        }
                    },
                    Err(e) => return Err(ZapataError::from(e)),
                }
            }
        }
        Ok(())
    }

    fn post_update(&mut self) -> Result<(), ZapataError> {
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), ZapataError> {
        let start = time::SystemTime::now();
        if let Err(e) = self.update_entities() {
            return Err(e);
        } else {
            self.ticks += 1;
        }
        let end = time::SystemTime::now();

       match end.duration_since(start) {
           Ok(dur) => self.total_tick_time += dur,
           Err(e) => return Err(ZapataError::from(e)),
       }

        println!("total: {:?}", self.total_tick_time);
        Ok(())
    }

    pub fn run(&mut self, epochs: usize) -> Result<(), ZapataError> {
        for epoch in 0..epochs {
            if let Err(e) = self.update() {
                return Err(e)
            }
        }
        Ok(())
    }
}

impl Scene {
    pub fn new(name: Option<String>) -> Self {
        let entities = HashMap::new();
        match name {
            Some(name) => Self {
                name,
                total_tick_time: time::Duration::default(),
                total_delta_tick_time: time::Duration::default(),
                ticks: 0,
                ids: 0,
                entities,
            },
            None => Self {
                name: String::from(DEFAULT_NAME),
                total_tick_time: time::Duration::default(),
                total_delta_tick_time: time::Duration::default(),
                ticks: 0,
                ids: 0,
                entities,
            }
        }
    }

    pub fn get_gravity(&self) -> Effect {
        Effect::new(String::from("Gravity"), Vec3::new(0.0, -9.821, 0.0), None)
    }

    pub fn average_tick(&self) -> Option<time::Duration> {
        if self.ticks == 0 || self.total_tick_time.is_zero() {
            return None
        }
        Some(self.total_tick_time.div_f64(self.ticks as f64))
    }

    pub fn average_delta_tick(&self) -> time::Duration {
        if self.ticks == 0 || self.total_delta_tick_time.is_zero() {
            return self.total_delta_tick_time
        }
        self.total_delta_tick_time.div_f64(self.ticks as f64)
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl Debug for Scene {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.get_name())
            .field("ticks", &self.ticks)
            .field("avg_tick", &self.average_tick())
            .field("avg_∆tick", &self.average_delta_tick())
            .field("entities", &self.entities.len())
            .finish()
    }
}