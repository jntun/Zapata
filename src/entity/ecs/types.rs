use crate::{
    entity::{ecs::ECS, Entity},
    error::ZapataError,
    physics::vec3::Vec3,
};

// Where "types" of entities are constructed & then returned to use

impl ECS {
    pub fn create_human(&mut self, pos: Option<Vec3>) -> Result<Entity, ZapataError> {
        let human: Entity;
        match self.get_next_entity() {
            Ok(entity) => human = entity,
            Err(e) => return Err(e),
        }

        if let Err(e) = self.physics.fill_new_entity(
            &human,
            crate::entity::component::physics::Physics::new(21.0, pos, None),
        ) {
            return Err(e);
        }

        if let Err(e) = self.collider.fill_new_entity(
            &human,
            crate::entity::component::collider::Collider::human(),
        ) {
            return Err(e);
        }

        if let Err(e) = self.health.fill_new_entity(
            &human,
            crate::entity::component::health::Health::new(100, Some(75)),
        ) {
            return Err(e);
        }

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
