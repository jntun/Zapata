use {
    crate::{
        entity::{component::*, Entity},
        error::ZapataError,
        physics::vec3::Vec3,
        scene::{tracked, Scene},
    },
    std::{cell::RefCell, rc::Rc},
};

const COMPONENT_NAME: &str = "Hitbox";

#[derive(Debug)]
pub struct Hitbox(pub Vec3);

impl Component for Hitbox {
    fn update(&mut self, entity: Entity, scene: &Scene) -> Result<(), ZapataError> {
        let do_physx = |e: &tracked::TrackedComponent| -> bool {
            match e {
                tracked::TrackedComponent::Physics(physx) => {
                    todo!("{:?}", physx)
                },
                _ => return false,
            }
            true
        };

        match scene.act_on_component_for_entity(entity, do_physx) {
            Ok(()) => return Ok(()),
            Err(e) => return Err(ZapataError::FatalError(String::from(format!("hitbox update failed: {:?}", e)))),
        }
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_name(&self) -> &str {
        COMPONENT_NAME
    }
}
