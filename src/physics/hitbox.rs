use crate::physics::vec3::Vec3;

#[derive(Debug)]
pub struct Hitbox {
    min: Vec3,
    max: Vec3,
}

impl Hitbox {
    pub fn intersects(&self, hitbox: Self) -> bool {
        (self.min.x() <= hitbox.max.x() && self.max.x() >= hitbox.min.x())
            && (self.min.y() <= hitbox.max.y() && self.max.y() >= hitbox.min.y())
            && (self.min.z() <= hitbox.max.z() && self.max.z() >= hitbox.min.z())
    }

    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
}
