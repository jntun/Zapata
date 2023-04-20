use crate::physics::{Effect, Vec3};

#[derive(Copy, Clone, Debug)]
pub enum Duration {
    Indefinite,
    EndTick(u64),
}

impl Effect {
    pub fn get_force(&self) -> Vec3 {
        self.force
    }

    pub fn is_alive(&self, current_tick: u64) -> bool {
        match self.duration {
            Duration::Indefinite => true,
            Duration::EndTick(end) => current_tick >= end,
        }
    }

    pub fn new(name: String, force: Vec3, duration: Option<Duration>) -> Self {
        if let Some(duration) = duration {
            Self {
                name,
                force,
                duration,
            }
        } else {
            Self {
                name,
                force,
                duration: Duration::Indefinite,
            }
        }
    }
}
