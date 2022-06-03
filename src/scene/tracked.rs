use {
    crate::{
        entity::{component, component::Component, Entity},
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
    Vague(Rc<RefCell<Box<dyn Component>>>),
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

impl From<component::physics::Physics> for TrackedComponent {
    fn from(component: component::physics::Physics) -> Self {
        Self::Physics(Rc::new(RefCell::new(component)))
    }
}

impl From<component::health::Health> for TrackedComponent {
    fn from(component: component::health::Health) -> Self {
        Self::Health(Rc::new(RefCell::new(component)))
    }
}

impl From<component::collider::Collider> for TrackedComponent {
    fn from(component: component::collider::Collider) -> Self {
        Self::Collider(Rc::new(RefCell::new(component)))
    }
}
