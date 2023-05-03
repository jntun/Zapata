use crate::{
    entity::{component, ecs::ECS},
    error::ZapataError,
    physics,
    scene::Scene,
};

pub trait System {
    fn run_system(&mut self, scene: &mut Scene) -> Result<(), ZapataError>;
}

pub struct Physics();

impl System for Physics {
    fn run_system(&mut self, scene: &mut Scene) -> Result<(), ZapataError> {
        if let Err(e) = do_physx_collider_entity_effects(&mut scene.ecs) {
            return Err(e);
        }
        let physx_effs = scene.physx_effects().clone();
        for entity in scene.ecs.entities.iter() {
            if let Some(physx) = scene.ecs.physics.get_mut(entity) {
                for effect in physx_effs.iter() {
                    physx.add_effect(effect.clone())
                }
            }
        }
        Ok(())
    }
}

fn do_physx_collider_entity_effects(ecs: &mut ECS) -> Result<(), ZapataError> {
    for (i, self_entity) in ecs.entities.iter().enumerate() {
        let self_collider: &component::Collider;
        let self_physx: &component::Physics;

        let mut effects: Vec<physics::Effect> = Vec::new();
        match (ecs.collider.get(self_entity), ecs.physics.get(self_entity)) {
            (Some(collider), Some(physx)) => {
                self_collider = collider;
                self_physx = physx;
            }
            _ => continue, // self_entity does not have Physics or Collider component, so we skip
        }

        for target_entity in ecs.entities.iter() {
            if self_entity == target_entity {
                continue;
            }

            match (
                ecs.collider.get(target_entity),
                ecs.physics.get(target_entity),
            ) {
                (Some(target_collider), Some(target_physx)) => {
                    match self_collider.collide_with(
                        target_collider,
                        self_physx.position(),
                        target_physx.position(),
                    ) {
                        Ok(mut collide_effects) => {
                            effects.append(&mut collide_effects);
                        }
                        Err(e) => return Err(e),
                    }
                }
                _ => continue,
            }
        }

        let Some(self_physx) = ecs.physics.get_mut(self_entity) else {
            return Err(ZapataError::RuntimeError(format!("Could not get mutable physics for {:?}", self_entity)))
        };

        for effect in effects {
            self_physx.add_effect(effect)
        }
    }

    Ok(())
}
