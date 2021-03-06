use {
    crate::{
        entity::{component::Component, Entity},
        error::ZapataError,
        physics::{effect::Effect, vec3::Vec3},
        scene::Scene,
    },
    std::fmt::{Debug, Formatter},
};

pub const COMPONENT_NAME: &str = "Physics";

#[derive(Clone)]
pub struct Physics {
    on: bool,
    mass: f64,
    momentum: Vec3,
    position: Vec3,
    effects: Vec<Effect>,
}

impl Physics {
    pub fn new(mass: f64, position: Option<Vec3>, start_on: Option<bool>) -> Self {
        let mut on = true;
        if let Some(o) = start_on {
            on = o;
        }
        match position {
            Some(position) => Self {
                on,
                mass,
                momentum: Vec3::new(0.0, 0.0, 0.0),
                position,
                effects: Vec::new(),
            },
            None => Self {
                on,
                mass,
                momentum: Vec3::new(0.0, 0.0, 0.0),
                position: Vec3::new(0.0, 0.0, 0.0),
                effects: Vec::new(),
            },
        }
    }

    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn effects_force_sum(&self) -> Vec3 {
        let mut total = Vec3::default();
        for effect in self.effects.iter() {
            total += effect.get_force();
        }
        total
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }
}

impl Component for Physics {
    fn update(&mut self, entity: Entity, scene: &Scene) -> Result<(), ZapataError> {
        let mut force = self.effects_force_sum();
        for scene_effect in scene.physics_effects.iter() {
            force += scene_effect.get_force();
        }

        self.momentum += force;
        self.position += self.momentum / self.mass;

        Ok(())
    }

    fn is_active(&self) -> bool {
        self.on
    }

    fn get_name(&self) -> &str {
        COMPONENT_NAME
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            on: true,
            mass: 1.0,
            momentum: Vec3::default(),
            position: Vec3::default(),
            effects: Vec::new(),
        }
    }
}

impl Debug for Physics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("data")
            .field("mass", &self.mass)
            .field("momentum", &self.momentum)
            .field("position", &self.position)
            .field("effects", &self.effects)
            .finish()
    }
}
