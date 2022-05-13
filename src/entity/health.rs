pub trait Health {
    fn do_damage(&mut self, dmg: u64);
    fn get_current(&self) -> u64;
    fn get_max(&self) -> u64;
}

#[derive(Debug, Clone)]
pub struct HealthTracker {
    current_health: u64,
    max_health: u64,
}

impl HealthTracker {
    pub fn new(max_health: u64, start_health: Option<u64>) -> Self {
        match start_health {
            Some(current_health) => Self {
                current_health,
                max_health,
            },
            None => Self {
                current_health: max_health,
                max_health,
            }
        }
    }
}

impl Health for HealthTracker {
    fn do_damage(&mut self, dmg: u64) {
        let dmg_done = dmg;
        self.current_health -= dmg_done;
    }

    fn get_current(&self) -> u64 {
        self.current_health
    }
    fn get_max(&self) -> u64 {
       self.max_health
    }
}