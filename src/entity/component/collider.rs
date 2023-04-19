use crate::physics::vec3::Vec3;
use crate::{
    entity::{
        component,
        component::{Collider, Component},
        Entity,
    },
    error::ZapataError,
    physics::hitbox::Hitbox,
};

impl Collider {
    fn do_collide(
        &self,
        self_physx: component::Physics,
        self_entity: Entity,
        target_physx: component::Physics,
        target_collider: &Hitbox,
        entity: Entity,
    ) -> Result<(), ZapataError> {
        Ok(())
    }

    fn try_collide(
        self,
        self_physx: component::Physics,
        self_entity: Entity,
        target_entity: Entity,
        target_physx: component::Physics,
        target_collider: Collider,
    ) -> Result<(), ZapataError> {
        return Ok(());
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
