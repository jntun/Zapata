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
        for index in 0..scene.entity_list_end().index() {
            //println!("{:?}", scene.component_list_for_entity(Entity::from(index)));
        }

        Ok(())
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_name(&self) -> &str {
        COMPONENT_NAME
    }
}
