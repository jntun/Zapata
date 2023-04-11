use crate::{
    entity::{component::Component, Entity},
    error::ZapataError,
    scene::Scene,
};

pub struct Printer(bool);

impl Component for Printer {
    fn update(&mut self, entity: Entity, scene: &Scene) -> Result<(), ZapataError> {
        println!("{:?}", entity);
        Ok(())
    }
}

impl Printer {
    pub fn new() -> Self {
        Self(true)
    }
}
