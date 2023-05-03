use {
    crate::{
        entity::{
            component::{Component, Physics},
            Entity,
        },
        error::ZapataError,
        physics::{Effect, Vec3},
    },
    std::fmt::{Debug, Formatter},
};

impl Physics {
    pub fn new(mass: f64, position: Option<Vec3>, start_on: bool) -> Self {
        match position {
            Some(position) => Self {
                on: start_on,
                mass,
                momentum: Vec3::new(0.0, 0.0, 0.0),
                position,
                effects: Vec::new(),
            },
            None => Self {
                on: start_on,
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
    fn update(&mut self, entity: Entity) -> Result<(), ZapataError> {
        let mut force = self.effects_force_sum();
        self.momentum += force;
        self.position += self.momentum / self.mass;
        self.effects.clear();
        Ok(())
    }

    fn name(&self) -> &str {
        "physx"
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
