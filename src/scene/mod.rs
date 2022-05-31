pub(crate) mod tracked;

use {
    crate::{
        entity::Entity,
        error::ZapataError,
        physics::{
            effect::{Duration, Effect},
            vec3::Vec3,
        },
    },
    std::{
        cell::RefCell,
        fmt::{Debug, Formatter},
        rc::Rc,
        time,
    },
    tracked::TrackedComponent,
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
    name: String,
    stats: SceneStats,
    pub physics_effects: Vec<Effect>,
    entities: Vec<Vec<TrackedComponent>>,
}

impl Scene {
    pub fn act_on_component_for_entity<T: Fn(&TrackedComponent) -> bool>(
        &self,
        entity: Entity,
        act: T,
    ) -> Result<(), ZapataError> {
        if let Some(comp_list) = self.component_list_for_entity(entity) {
            for component in comp_list.into_iter() {
                if act(component) { return Ok(()) }
            }
        }
        Err(ZapataError::RuntimeError(String::from(format!("couldn't act on {:?}", entity))))
    }

    fn component_list_for_entity(&self, entity: Entity) -> Option<&Vec<TrackedComponent>> {
        self.entities.get(entity.index())
    }

    pub fn add_entity(&mut self, components: Vec<TrackedComponent>) -> Result<Entity, ZapataError> {
        self.entities.push(components);
        Ok(Entity(self.entities.len() - 1))
    }

    fn pre_update(&mut self) -> Result<(), ZapataError> {
        Ok(())
    }

    fn update_entities(&mut self) -> Result<(), ZapataError> {
        for (i_as_e, comp_list) in self.entities.iter().enumerate() {
            for comp in comp_list.iter() {
                if let Err(e) = comp.update(Entity(i_as_e), self) {
                    return Err(e);
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