use {
    std::{
        fmt::{Debug, Formatter},
        cell::RefCell,
        rc::Rc,
        time,
    },
    crate::{
        entity::{
            component::Component,
            Entity,
        },
        error:: ZapataError,
        physics::{
            effect::{Effect, Duration},
            vec3::Vec3,
        },
    }
};

const DEFAULT_NAME: &str = "Zapata";

#[derive(Default)]
struct SceneStats {
    total_tick_time:       time::Duration,
    total_delta_tick_time: time::Duration,
    last_tick_duration:    time::Duration,
    ticks:                 u64,
}

pub struct Scene {
    name:                  String,
    stats:                 SceneStats,
    entities:              Vec<Vec<Rc<RefCell<Box<dyn Component>>>>>,
}

impl Scene {
   pub fn add_entity(&mut self, components: Vec<Box<dyn Component>>) -> Result<Entity, ZapataError> {
        let comp_list = components.into_iter().map(|c| Rc::new(RefCell::new(c))).collect();
        self.entities.push(comp_list);
        Ok(Entity::from(self.entities.len()-1))
    }

    fn pre_update(&mut self) -> Result<(), ZapataError> {
        Ok(())
    }

    fn update_entities(&mut self) -> Result<(), ZapataError> {
        for (i_as_e, comp_list) in self.entities.iter().enumerate() {
            for comp in comp_list.iter() {
                match comp.try_borrow_mut() {
                    Ok(mut comp) => {
                        match comp.update(Entity::from(i_as_e), self) {
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

    fn update(&mut self) -> Result<(), ZapataError> {
        let start = time::SystemTime::now();
        if let Err(e) = self.update_entities() {
            return Err(e);
        } else {
            self.stats.ticks += 1;
        }
        let end = time::SystemTime::now();

       match end.duration_since(start) {
           Ok(dur) => {
               self.stats.total_tick_time += dur;

               if dur > self.stats.last_tick_duration { // If this tick took longer than the previous, add the dur to the total delta time
                   self.stats.total_delta_tick_time += dur - self.stats.last_tick_duration
               }

               self.stats.last_tick_duration = dur;
           },
           Err(e) => return Err(ZapataError::from(e)),
       }

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
        let mut scene = Scene::default();
        if let Some(name) = name {
            scene.name = name;
        }
        scene
    }

    pub fn get_gravity(&self) -> Effect {
        Effect::new(String::from("Gravity"), Vec3::new(0.0, -9.821, 0.0), None)
    }

    pub fn average_tick(&self) -> Option<time::Duration> {
        if self.stats.ticks == 0 || self.stats.total_tick_time.is_zero() {
            return None
        }
        Some(self.stats.total_tick_time.div_f64(self.stats.ticks as f64))
    }

    pub fn average_delta_tick(&self) -> time::Duration {
        if self.stats.ticks == 0 || self.stats.total_delta_tick_time.is_zero() {
            return self.stats.total_delta_tick_time
        }
        self.stats.total_delta_tick_time.div_f64(self.stats.ticks as f64)
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl Debug for Scene {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.get_name())
            .field("ticks", &self.stats.ticks)
            .field("avg_tick", &self.average_tick())
            .field("avg_∆tick", &self.average_delta_tick())
            .field("entities", &self.entities.len())
            .finish()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            name: String::from(DEFAULT_NAME),
            stats: SceneStats::default(),
            physics_effects: vec![Effect::new(String::from("Gravity", ), Vec3::new(0.0, -9.821, 0.0), Some(Duration::Indefinite))],
            entities: Vec::new(),
        }
    }
}