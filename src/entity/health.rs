use super::Entity;

pub(crate) type Health = u64;
pub(crate) type Damage = u64;

pub trait Healther: Entity {
    fn do_damage(&mut self, dmg: Damage);
    fn set_max(&mut self, health: Health);
    fn get_current(&self) -> Health;
    fn get_max(&self) -> Health;
}

pub trait Attacker: Entity {
    fn attack<T: Healther>(&mut self, target: &mut T)  -> Damage;
    fn get_damage(&self) -> Damage;
}

#[derive(Debug, Clone, Copy)]
pub struct HealthStat {
    pub current: Health,
    pub max: Health,
}

impl HealthStat {
    pub fn new(max: Health, start: Option<Health>) -> Self {
      match start {
          Some(current) => Self {current, max},
          None => Self {current: max, max},
      }
    }
}


impl std::ops::Add<Health> for HealthStat {
    type Output = Health;

    fn add(self, rhs: Health) -> Health {
        self.current + rhs
    }
}

impl std::ops::AddAssign<Health> for HealthStat {
    fn add_assign(&mut self, rhs: Health) {
        self.current += rhs;
    }
}

impl std::ops::SubAssign<Health> for HealthStat {
    fn sub_assign(&mut self, rhs: Health) {
        self.current -= rhs;
    }
}

impl Default for HealthStat {
    fn default() -> Self {
        Self {
            current: 100,
            max: 100,
        }
    }
}