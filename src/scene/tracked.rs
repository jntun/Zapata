use {
    crate::{
        entity::{component, Entity},
        error::ZapataError,
        scene::Scene,
    },
    std::{cell::RefCell, rc::Rc},
};

#[derive(Debug)]
pub enum TrackedComponent {
    Physics(Rc<RefCell<component::physics::Physics>>),
    Collider(Rc<RefCell<component::collider::Collider>>),
    Health(Rc<RefCell<component::health::Health>>),
    Vague(Rc<RefCell<Box<dyn component::Component>>>),
}

impl TrackedComponent {
    pub fn update(&self, entity: Entity, scene: &Scene) -> Result<(), ZapataError> {
        match self {
            TrackedComponent::Physics(e) => e.borrow_mut().update(entity, scene),
            TrackedComponent::Collider(e) => e.borrow_mut().update(entity, scene),
            TrackedComponent::Health(e) => e.borrow_mut().update(entity, scene),
            TrackedComponent::Vague(e) => e.borrow_mut().update(entity, scene),
        }
    }
}
