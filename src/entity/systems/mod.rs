use crate::{
    entity::{component, ecs::ECS},
    error::ZapataError,
    physics,
};

impl ECS {
    pub fn do_physx_effects(
        &mut self,
        scene_effects: &Vec<physics::Effect>,
    ) -> Result<(), ZapataError> {
        for (i, self_entity) in self.entities.iter().enumerate() {
            let self_collider: &component::Collider;
            let self_physx: &component::Physics;

            let mut effects: Vec<physics::Effect> = Vec::new();
            match (
                self.collider.get(self_entity),
                self.physics.get(self_entity),
            ) {
                (Some(collider), Some(physx)) => {
                    self_collider = collider;
                    self_physx = physx;
                }
                _ => continue, // self_entity does not have Physics or Collider component, so we skip
            }

            for target_entity in self.entities.iter() {
                if self_entity == target_entity {
                    continue;
                }

                match (
                    self.collider.get(target_entity),
                    self.physics.get(target_entity),
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

            let Some(self_physx) = self.physics.get_mut(self_entity) else {
                return Err(ZapataError::RuntimeError(format!("Could not get mutable physics for {:?}", self_entity)))
            };
            for scene_effect in scene_effects {
                self_physx.add_effect(scene_effect.clone())
            }
            for effect in effects {
                self_physx.add_effect(effect)
            }
        }

        Ok(())
    }
}
