use {
    std::{
        fmt::{Formatter, Debug, Display},
    },
    super::{
        Entity,
        health::{Healther, Attacker, HealthStat, Health, Damage},
    },
    crate::{
        World,
        physics::{PhysicsEntity, PhysxComponent, vec3::Vec3},
        error::TickError,
    },
};

const DEFAULT_HEALTH: u64 = 100;
const DEFAULT_MASS:   f64 = 20.0;
const DEFAULT_NAME:  &str = "default player";

#[derive(Clone)]
#[allow(dead_code)]
pub struct Human {
    name:   String,
    health: HealthStat,
    physx: PhysxComponent,
}

impl Human {
    pub fn new(max_health: u64, start_health: Option<Health>, name: Option<String>, position: Option<Vec3>) -> Self {
        match name {
            Some(name) => Self {
                name,
                health: HealthStat::new(max_health, start_health),
                physx: PhysxComponent::new(DEFAULT_MASS, position),
            },
            None => Self {
                name: String::from(DEFAULT_NAME),
                health: HealthStat::new(max_health, start_health),
                physx: PhysxComponent::new(DEFAULT_MASS, position),
            }
        }
    }
}

impl PhysicsEntity for Human {
    fn get_physx_data(&self) -> &PhysxComponent {
        &self.physx
    }

    fn mut_physx_data(&mut self) -> &mut PhysxComponent {
        &mut self.physx
    }
}

impl Attacker for Human {
    fn attack<T: Healther>(&mut self, target: &mut T) {
        target.do_damage(self.get_damage());
        println!("{} just attacked '{}' for {} dmg", self, target.get_name(), self.get_damage());
    }

    fn get_damage(&self) -> Damage {
        1 as u64
    }
}

impl Healther for Human {
    fn do_damage(&mut self, dmg: Damage) {
        self.health -= dmg;
    }

    fn set_max(&mut self, health: Health) {
        self.health.max = health;
    }

    fn get_current(&self) -> Health {
        self.health.current
    }

    fn get_max(&self) -> Health {
        self.health.max
    }
}

impl Entity for Human {
    fn tick(&mut self, _world: &World) -> Result<(), TickError> {
        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl Display for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} - {}", self.name, self.get_current()))
    }
}

impl Debug for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(self.get_name())
            .field("health", &self.get_current())
            .field("max_health", &self.get_max())
            .field("physx", self.get_physx_data())
            .finish()
    }
}

impl Default for Human {
    fn default() -> Self {
        Self {
            name: String::from(DEFAULT_NAME),
            health: HealthStat::new(DEFAULT_HEALTH, None),
            physx: PhysxComponent::new(DEFAULT_MASS, None),
        }
    }
}
