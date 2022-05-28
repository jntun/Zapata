mod scene;
mod entity;
mod error;
mod physics;

use crate::{
    entity::*,
    scene::Scene,
    error::ZapataResult,
    physics::vec3::Vec3,
};

const EPOCH_COUNT: usize = 1000;
const HUMAN_COUNT: usize = 20;



fn main() -> error::ZapataResult {
    let mut scene = Scene::new(None);

    for i in 0..HUMAN_COUNT {
        let mut comp: Vec<Box<dyn component::Component>> = Vec::new();
        let x = i as f64;
        comp.append(&mut vec![
            Box::new(component::health::Health::new((i * 10) as i64, Some((i * 5) as i64))),
            Box::new(component::physics::Physics::new(21.0, Some(Vec3::new(x, x, x)), None)),
        ]);

        match scene.add_entity(comp) {
            Ok(e) => (),//println!("entity added {:?}", e),
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