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

    fn is_active(&self) -> bool {
        self.0
    }

    fn get_name(&self) -> &str {
        "Printer"
    }
}

impl Printer {
    pub fn new() -> Self {
        Self(true)
    }
}
