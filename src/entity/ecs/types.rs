use crate::{
    entity::{
        component::{collider, health, physics},
        ecs::ECS,
        ecs::{ComponentEntry, OccupiedComponent},
        Entity,
    },
    error::ZapataError,
    physics::vec3::Vec3,
};
use std::ops::Index;

// Where "types" of entities are constructed & then returned to use

impl ECS {
    pub fn create_human(&mut self, pos: Option<Vec3>) -> Result<Entity, ZapataError> {
        let human: Entity;
        match self.get_next_entity() {
            Ok(entity) => human = entity,
            Err(e) => return Err(e),
        }

        self.physics.insert(
            human.index,
            ComponentEntry::Occupied(OccupiedComponent {
                component: physics::Physics::new(21.0, pos, None),
                generation: human.generation,
            }),
        );

        self.collider.insert(
            human.index,
            ComponentEntry::Occupied(OccupiedComponent {
                generation: human.generation,
                component: collider::Collider::human(),
            }),
        );

        self.health.insert(
            human.index,
            ComponentEntry::Occupied(OccupiedComponent {
                generation: human.generation,
                component: health::Health::new(100, Some(75)),
            }),
        );

        self.entities.push(human);
        Ok(human)
    }

    pub fn create_bullet(&mut self, pos: Vec3) -> Result<Entity, ZapataError> {
        let bullet: Entity;
        match self.get_next_entity() {
            Ok(e) => bullet = e,
            Err(e) => return Err(e),
        }

        Ok(bullet)
    }
}
