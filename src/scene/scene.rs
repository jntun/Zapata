use {
    crate::{
        entity::Entity,
        error::ZapataError,
        physics::{
            effect::{Duration, Effect},
            vec3::Vec3,
        },
        scene::{tracked::TrackedComponent, Scene},
    },
    std::{
        fmt::{Debug, Formatter},
        time,
    },
};

const DEFAULT_NAME: &str = "Zapata";

impl Scene {
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

    pub fn update(&mut self) -> Result<(), ZapataError> {
        self.lifetime.start();
        if let Err(e) = self.pre_update() {
            return Err(e);
        }
        if let Err(e) = self.update_entities() {
            return Err(e);
        }
        if let Err(e) = self.post_update() {
            return Err(e);
        }
        self.lifetime.stop();
        Ok(())
    }
}

impl Scene {
    pub fn component_list_for_entity(&self, entity: Entity) -> Option<&Vec<TrackedComponent>> {
        self.entities.get(entity.index())
    }

    pub fn entity_list_end(&self) -> Entity {
        Entity::from(self.entities.len())
    }

    pub fn add_entity(&mut self, components: Vec<TrackedComponent>) -> Result<Entity, ZapataError> {
        self.entities.push(components);
        Ok(Entity(self.entities.len()))
    }

    pub fn current_tick(&self) -> u64 {
        self.lifetime.ticks
    }

    pub fn new(name: Option<String>) -> Self {
        let mut scene = Scene::default();
        if let Some(name) = name {
            scene.name = name;
        }
        scene
    }

    pub fn average_tick(&self) -> Option<time::Duration> {
        if self.lifetime.ticks == 0 || self.lifetime.total_tick_time.is_zero() {
            return None;
        }
        Some(
            self.lifetime
                .total_tick_time
                .div_f64(self.lifetime.ticks as f64),
        )
    }

    pub fn average_delta_tick(&self) -> time::Duration {
        if self.lifetime.ticks == 0 || self.lifetime.total_delta_tick_time.is_zero() {
            return self.lifetime.total_delta_tick_time;
        }
        self.lifetime
            .total_delta_tick_time
            .div_f64(self.lifetime.ticks as f64)
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl Debug for Scene {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.get_name())
            .field("ticks", &self.lifetime.ticks)
            .field("runtime", &self.lifetime.total_tick_time)
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
            lifetime: super::Lifetime::default(),
            physics_effects: vec![Effect::new(
                String::from("Gravity"),
                Vec3::new(0.0, -9.821, 0.0),
                Some(Duration::Indefinite),
            )],
            entities: Vec::new(),
        }
    }
}
