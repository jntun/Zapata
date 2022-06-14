use crate::{
    entity::{component::*, Entity},
    error::ZapataError,
    physics,
    scene::{tracked, Scene},
};

const COMPONENT_NAME: &str = "Hitbox";

#[derive(Debug)]
pub struct Collider(pub Vec<physics::hitbox::Hitbox>);

impl Collider {}

impl Component for Collider {
    fn update(&mut self, self_entity: Entity, scene: &Scene) -> Result<(), ZapataError> {
        let collide = |e: &tracked::TrackedComponent| -> bool {
            return true;
        };

        return match scene.act_on_all_other_entities(self_entity, collide) {
            Ok(entities) => Ok(()),
            Err(e) => Err(e),
        };
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_name(&self) -> &str {
        COMPONENT_NAME
    }
}
