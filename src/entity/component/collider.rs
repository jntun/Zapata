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
    fn update(&mut self, entity: Entity, scene: &Scene) -> Result<(), ZapataError> {
        let collide = |e: &tracked::TrackedComponent| -> bool {
            match e {
                tracked::TrackedComponent::Collider(collider) => {
                    todo!("{:?}", collider)
                }
                _ => return false,
            }
            true
        };

        return match scene.act_on_component_for_entity(entity, collide) {
            Ok(()) => Ok(()),
            Err(e) => Err(ZapataError::FatalError(String::from(format!(
                "hitbox update failed: {:?}",
                e
            )))),
        };
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_name(&self) -> &str {
        COMPONENT_NAME
    }
}
