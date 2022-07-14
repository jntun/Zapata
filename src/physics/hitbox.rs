use crate::physics::vec3::Vec3;

#[derive(Debug)]
pub struct Hitbox {
    min: Vec3,
    max: Vec3,
}

impl Hitbox {
    pub fn intersects(&self, self_pos: Vec3, target_pos: Vec3, target_hitbox: &Self) -> bool {
        // To check if a hitbox[self] is intersecting with another[target_hitbox], we need to know
        // both the position[self_pos, target_pos] of the hitbox and it's bounds dimensions[min, max]
        // and then add the bounds to the hitbox position.
        let self_hitbox_min = Vec3::new(
            self_pos.x() - self.min.x(),
            self_pos.y() - self.min.y(),
            self_pos.z() - self.min.z(),
        );
        let self_hitbox_max = Vec3::new(
            self_pos.x() + self.max.x(),
            self_pos.y() + self.max.y(),
            self_pos.z() + self.max.z(),
        );
        let target_hitbox_min = Vec3::new(
            target_pos.x() - target_hitbox.min.x(),
            target_pos.y() - target_hitbox.min.y(),
            target_pos.z() - target_hitbox.min.z(),
        );
        let target_hitbox_max = Vec3::new(
            target_pos.x() + target_hitbox.max.x(),
            target_pos.y() + target_hitbox.max.y(),
            target_pos.z() + target_hitbox.max.z(),
        );

        (self_hitbox_min.x() <= target_hitbox_max.x()
            && self_hitbox_max.x() >= target_hitbox_min.x())
            && (self_hitbox_min.y() <= target_hitbox_max.y()
                && self_hitbox_max.y() >= target_hitbox_min.y())
            && (self_hitbox_min.z() <= target_hitbox_max.z()
                && self_hitbox_max.z() >= target_hitbox_min.z())
    }

    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
}
