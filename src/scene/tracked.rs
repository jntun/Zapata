use {
    crate::{
        entity::{component, component::Component, Entity},
        error::ZapataError,
        physics::{self, vec3::Vec3},
        scene::Scene,
    },
    std::{cell::RefCell, rc::Rc},
};

#[derive(Debug)]
pub enum TrackedComponent {
    Physics(Rc<RefCell<component::physics::Physics>>),
    Collider(Rc<RefCell<component::collider::Collider>>),
    Health(Rc<RefCell<component::health::Health>>),
}

impl TrackedComponent {
    pub fn update(&self, entity: Entity, scene: &Scene) -> Result<(), ZapataError> {
        match self {
            TrackedComponent::Physics(e) => e.borrow_mut().update(entity, scene),
            TrackedComponent::Collider(e) => e.borrow_mut().update(entity, scene),
            TrackedComponent::Health(e) => e.borrow_mut().update(entity, scene),
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

pub fn human(position: Option<Vec3>) -> Vec<TrackedComponent> {
    vec![
        TrackedComponent::from(component::health::Health::new(100, None)),
        TrackedComponent::from(component::physics::Physics::new(21.0, position, None)),
        TrackedComponent::from(component::collider::Collider(vec![
            physics::hitbox::Hitbox::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0)),
        ])),
    ]
}
