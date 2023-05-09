use crate::{
    entity::{component, ecs::ECS, Entity},
    error::ZapataError,
    kit,
    physics::Vec3,
};

macro_rules! fill_new_entity_or_err {
    ($e:ident, $comp_arr:expr, $comp:expr) => {
        if let Err(e) = $comp_arr.fill_new_entity(&$e, $comp) {
            return Err(e);
        }
    };
}

// Where "types" of entities are constructed & then returned to use

impl ECS {
    pub fn create_human(
        &mut self,
        pos: Option<Vec3>,
        heading: Option<kit::Float2>,
    ) -> Result<Entity, ZapataError> {
        let human = match self.get_next_entity() {
            Ok(entity) => entity,
            Err(e) => return Err(e),
        };

        fill_new_entity_or_err!(
            human,
            self.physics,
            component::Physics::new(21.0, pos, heading, true)
        );
        fill_new_entity_or_err!(human, self.collider, component::Collider::human());
        fill_new_entity_or_err!(human, self.health, component::Health::new(100, Some(75)));

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
