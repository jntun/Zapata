use crate::physics::vec3::Vec3;
use {
    crate::{
        entity::{component, component::Component, Entity},
        error::ZapataError,
        physics,
        physics::hitbox::Hitbox,
        scene::{tracked, Scene},
    },
    std::{cell::RefCell, rc::Rc},
};

const COMPONENT_NAME: &str = "Collider";

#[derive(Debug)]
pub struct Collider(pub Vec<physics::hitbox::Hitbox>);

impl Collider {
    fn do_collide(
        &self,
        self_physx: component::physics::Physics,
        self_entity: Entity,
        target_physx: component::physics::Physics,
        target_collider: &Hitbox,
        entity: Entity,
    ) -> Result<(), ZapataError> {
        Ok(())
    }

    fn try_collide(
        self,
        self_physx: component::physics::Physics,
        self_entity: Entity,
        target_entity: Entity,
        target_physx: component::physics::Physics,
        target_collider: Collider,
    ) -> Result<(), ZapataError> {
        //println!("{} - {} - {:?}", self_entity, entity, self_physx);
        for target_hitbox in target_collider.0.into_iter() {
            for self_hitbox in self.0.iter() {
                if self_hitbox.intersects(
                    &self_physx.position(),
                    &target_physx.position(),
                    &target_hitbox,
                ) {
                    println!(
                        "{}: {:?} intersected {:?} ",
                        COMPONENT_NAME, self_entity, target_entity,
                    )
                    /*
                    match self.do_collide(
                        self_physx,
                        self_entity,
                        target_physx,
                        target_hitbox,
                        target_entity,
                    ) {
                        Ok(()) => (),
                        Err(e) => return Err(e),
                    }
                     */
                }
            }
        }
        return Ok(());
    }

    pub fn human() -> Self {
        return Self {
            0: vec![Hitbox::new(
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
            )],
        };
    }
}

impl Component for Collider {
    fn update(&mut self, self_entity: Entity, scene: &Scene) -> Result<(), ZapataError> {
        /* March 27, 2023
        I think I want collision detection to happen all in 'one' loop instead of every update() call to a Collider component.
            My thinking is that it will function better to have the scene do all collision detections, add them to a CollisionEvent queue for the
            corresponding Collider components & then have the update() call to each Collider work through their respective queues
            by applying the CollisionEvents properly.
         */
        Ok(())
    }
}
