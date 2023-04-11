mod entity;
mod error;
mod physics;
mod scene;

use crate::scene::SceneManager;
use {
    crate::{
        entity::*,
        error::ZapataResult,
        physics::vec3::Vec3,
        scene::{tracked, Scene},
    },
    std::rc::Rc,
};

const EPOCH_COUNT: usize = 100;
const HUMAN_COUNT: usize = 100;

fn main() -> ZapataResult {
    let mut scene = Scene::new(None);
    for i in 0..HUMAN_COUNT {
        match scene.ecs.create_human(Some(Vec3::new(i as f64, 0.0, 0.0))) {
            Ok(e) => (),
            Err(e) => {
                println!("Couldn't add human to scene: {}", e);
                return ZapataResult::Fatal;
            }
        }
    }

    let mut manager = SceneManager::new(vec![scene]);

    match manager.run(Some(EPOCH_COUNT)) {
        Ok(()) => {
            print!("\nDone - Ran for {} ticks:\n\t{}", EPOCH_COUNT, manager);
            //println!("{:?}", manager);
            ZapataResult::Success
        }
        Err(e) => {
            eprintln!("{:?}", e);
            ZapataResult::from(e)
        }
    }
}
