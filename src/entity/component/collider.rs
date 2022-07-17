use {
    crate::{
        entity::{component, component::Component, Entity},
        error::ZapataError,
        physics,
        scene::{tracked, Scene},
    },
    std::{cell::RefCell, rc::Rc},
};

const COMPONENT_NAME: &str = "Collider";

#[derive(Debug)]
pub struct Collider(pub Vec<physics::hitbox::Hitbox>);

// FIXME: collider should only hold a single hitbox. if you want another hitbox, add another component.

impl Collider {
    fn try_collide(
        &self,
        self_physx: &Rc<RefCell<component::physics::Physics>>,
        self_entity: Entity,
        entity: Entity,
        scene: &Scene,
    ) -> Result<(), ZapataError> {
        let comp_list = scene.component_list_for_entity(entity);

        if let Some(comp_list) = comp_list {
            let mut target_physx: Option<&Rc<RefCell<component::physics::Physics>>> = None;
            let mut target_collider: Option<&Rc<RefCell<Collider>>> = None;

            for component in comp_list.into_iter() {
                match component {
                    tracked::TrackedComponent::Collider(collider) => {
                        target_collider = Some(collider)
                    }
                    tracked::TrackedComponent::Physics(physx) => target_physx = Some(physx),
                    _ => (),
                }
            }

            //println!("{} - {} - {:?}", self_entity, entity, self_physx);
            match (target_collider, target_physx) {
                (Some(target_collider), Some(target_physx)) => {
                    for target_hitbox in target_collider.borrow_mut().0.iter() {
                        for self_hitbox in self.0.iter() {
                            if self_hitbox.intersects(
                                self_physx.borrow().position(),
                                target_physx.borrow().position(),
                                target_hitbox,
                            ) {
                                println!(
                                    "{}: {:?} intersected {:?} - {}",
                                    COMPONENT_NAME,
                                    self_entity,
                                    entity,
                                    scene.current_tick()
                                )
                            }
                        }
                    }
                    return Ok(());
                }
                (None, Some(target_physx)) => {
                    return Err(ZapataError::RuntimeError(format!(
                        "{:?} did not have {} component",
                        entity, COMPONENT_NAME
                    )));
                }
                (Some(target_collider), None) => {
                    return Err(ZapataError::RuntimeError(format!(
                        "{:?} did not have {} component",
                        entity,
                        component::physics::COMPONENT_NAME
                    )))
                }
                _ => {
                    return Err(ZapataError::RuntimeError(format!(
                        "{}: {:?} could not find matching components",
                        entity, COMPONENT_NAME
                    )))
                }
            }
        }
        Err(ZapataError::RuntimeError(format!("done.")))
    }
}

// TODO: generic Component errors
impl Component for Collider {
    fn update(&mut self, self_entity: Entity, scene: &Scene) -> Result<(), ZapataError> {
        if let Some(self_comp_list) = scene.component_list_for_entity(self_entity) {
            for comp in self_comp_list.into_iter() {
                match comp {
                    tracked::TrackedComponent::Physics(physx) => {
                        for index in 0..scene.entity_list_end().index() {
                            let target_entity = Entity::from(index);
                            if target_entity == self_entity {
                                continue;
                            }
                            println!("{} doing loop {}", self_entity, target_entity);
                            //println!("{:?} - {:?}", self_entity, target_entity);
                            match self.try_collide(physx, self_entity, target_entity, scene) {
                                Ok(()) => (),
                                Err(e) => return Err(e),
                            };
                        }
                    }
                    _ => (),
                }
            }
        } else {
            return Err(ZapataError::RuntimeError(format!(
                "{}-{}: couldn't get component list.",
                COMPONENT_NAME, self_entity
            )));
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
