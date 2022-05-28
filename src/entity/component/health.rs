use crate::{
    entity::{component::Component, Entity},
    error::ZapataError,
    scene::Scene,
};

pub(crate) type HealthUnit = i64;

const COMPONENT_NAME: &str = "Health";

pub struct Damage {
    source: Entity,
    dest:   Entity,
    cause:  dyn Component,
}

pub struct Health {
    start:   HealthUnit,
    current: HealthUnit,
    max:     HealthUnit,
}


impl Component for Health {
    fn update(&mut self, entity: Entity, scene: &Scene) -> Result<(), ZapataError> {
        Ok(())
    }

    fn is_active(&self) -> bool { true }

    fn get_name(&self) -> &str { COMPONENT_NAME }
}

impl Health {
    pub fn new(max: HealthUnit, start: Option<HealthUnit>) -> Self {
      match start {
          Some(start) => Self {start, current: start, max},
          None => Self {start: max, current: max, max},
      }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0
    }
}


impl std::ops::Add<HealthUnit> for Health {
    type Output = HealthUnit;

    fn add(self, rhs: HealthUnit) -> HealthUnit {
        self.current + rhs
    }
}

impl std::ops::AddAssign<HealthUnit> for Health {
    fn add_assign(&mut self, rhs: HealthUnit) {
        self.current += rhs;
    }
}

impl std::ops::SubAssign<HealthUnit> for Health {
    fn sub_assign(&mut self, rhs: HealthUnit) {
        if self.current == 0 || !self.is_alive() { return }; // No health or currently dead
        let mut dmg = rhs;
        if self.current < rhs {
            // If we're about to take more damage than we have health, reduce the damage so it'll put us to 0
            dmg -= dmg - self.current
        }
        self.current -= dmg;
    }
}

impl Default for Health {
    fn default() -> Self {
        Self {
            start: 100,
            current: 100,
            max: 100,
        }
    }
}