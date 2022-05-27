mod scene;
mod entity;
mod error;
mod physics;

use scene::Scene;
use crate::entity::Component;
use crate::error::{ZapataError, ZapataResult};

const EPOCH_COUNT: usize = 100;
const HUMAN_COUNT: usize = 100000;



fn main() -> error::ZapataResult {
    let mut scene = Scene::new(None);

    for _ in 0..HUMAN_COUNT {
        let mut comp: Vec<Box<dyn Component>> = Vec::new();
        comp.append(&mut vec![
            Box::new(entity::health::Health::default()),
        ]);

        match scene.add_entity(comp) {
            Ok(e) => println!("entity added #{:?}", e),
            Err(e) => {
                println!("failed adding entity: {:?}", e);
            }
        }
    }

    match scene.run(EPOCH_COUNT) {
        Ok(()) => {
            print!("\nDone - Ran for {} ticks:\n\t", EPOCH_COUNT);
            println!("{:?}", scene);
            ZapataResult::Success
        },
        Err(e) => ZapataResult::from(e),
    }
}