use crate::{
    entity::{
        component::{Component, Printer},
        Entity,
    },
    error::ZapataError,
};

impl Component for Printer {
    fn update(&mut self, entity: &Entity) -> Result<(), ZapataError> {
        println!("{:?}", entity);
        Ok(())
    }

    fn name(&self) -> &str {
        "printer"
    }
}

impl Printer {
    pub fn new() -> Self {
        Self(true)
    }
}
