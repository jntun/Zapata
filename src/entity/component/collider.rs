use crate::{
    entity::{
        component,
        component::{Collider, Component},
        Entity,
    },
    error::ZapataError,
    physics::{Effect, Hitbox, Vec3},
};

impl Collider {
    pub fn collide_with(
        &self,
        target: &Collider,
        self_pos: Vec3,
        target_pos: Vec3,
    ) -> Result<Vec<Effect>, ZapataError> {
        for self_hitbox in self.hitbox.iter() {
            for target_hitbox in target.hitbox.iter() {
                if self_hitbox.intersects(self_pos, target_pos, target_hitbox) {}
            }
        }
        Ok(vec![])
    }

    pub fn human() -> Self {
        return Self {
            hitbox: vec![Hitbox::new(
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
            )],
        };
    }
}

impl Component for Collider {
    fn update(&mut self, self_entity: Entity) -> Result<(), ZapataError> {
        /* March 27, 2023
        I think I want collision detection to happen all in 'one' loop instead of every update() call to a Collider component.
            My thinking is that it will function better to have the scene do all collision detections, add them to a CollisionEvent queue for the
            corresponding Collider components & then have the update() call to each Collider work through their respective queues
            by applying the CollisionEvents properly.
         */
        Ok(())
    }

    fn name(&self) -> &str {
        "collider"
    }
}
