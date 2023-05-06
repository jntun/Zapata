use crate::physics::{Effect, Vec3};

impl Effect {
    pub fn force(&self) -> &Vec3 {
        &self.force
    }

    pub fn new(force: Vec3) -> Self {
        Self { force }
    }
}
