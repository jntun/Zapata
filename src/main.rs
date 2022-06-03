mod entity;
mod error;
mod physics;
mod scene;

use crate::{
    entity::*,
    error::ZapataResult,
    physics::vec3::Vec3,
    scene::{tracked, Scene},
};
use std::{cell::RefCell, rc::Rc};

const EPOCH_COUNT: usize = 1000;
const HUMAN_COUNT: usize = 20;

fn main() -> error::ZapataResult {
    let mut scene = Scene::new(None);

    for i in 0..HUMAN_COUNT {
        let pos = i as f64;
        match scene.add_entity(scene::tracked::human(Some(Vec3::new(pos, pos, pos)))) {
            Ok(e) => (), //println!("entity added {:?}", e),
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
        }
        Err(e) => ZapataResult::from(e),
    }
}
