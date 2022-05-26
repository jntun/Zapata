use {
    std::rc::Rc,
    crate::{
        entity::{Component, Entity},
        error::ZapataError,
        scene::Scene,
    }
};

pub(crate) type HealthUnit = i64;
pub(crate) type DamageUnit = i64;

const COMPONENT_NAME: &str = "Health";

struct Damage {
    source: Entity,
    dest:   Entity,
    cause: Rc<dyn Component>,
}

struct Health {
    start:   HealthUnit,
    current: HealthUnit,
    max:     HealthUnit,
}


impl Component for Health {
    fn update(&mut self, entity: Entity, scene: &Scene) -> Result<(), ZapataError> {
        Ok(())
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_name(&self) -> &str {
        COMPONENT_NAME
    }
}

impl Health {
    pub fn new(max: HealthUnit, start: Option<HealthUnit>) -> Self {
      match start {
          Some(start) => Self {start, current: start, max},
          None => Self {start: max, current: max, max},
      }
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
        self.current -= rhs;
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