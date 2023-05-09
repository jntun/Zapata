mod entity;
mod error;
mod kit;
mod physics;
mod scene;

use crate::{
    error::ZapataResult,
    scene::{ManagedScene, Scene, SceneManager},
};

const EPOCH_COUNT: usize = 100;
const HUMAN_COUNT: usize = 100;

fn main() -> ZapataResult {
    let mut scene = Scene::new(None);
    for i in 0..HUMAN_COUNT {
        match scene
            .ecs
            .create_human(Some(physics::Vec3::new(i as f64, 0.0, 0.0)))
        {
            Ok(e) => (),
            Err(e) => {
                println!("Couldn't add human to scene: {}", e);
                return ZapataResult::Fatal;
            }
        }
    }
    let mut manager = SceneManager::new(vec![ManagedScene::new(
        scene,
        entity::systems::Registry::new().enable_physics().done(),
    )]);

    match manager.run(Some(EPOCH_COUNT)) {
        Ok(()) => {
            print!("\nDone - Ran for {} ticks:\n\t{:?}", EPOCH_COUNT, manager);
            //println!("{:?}", manager);
            ZapataResult::Success
        }
        Err(e) => {
            eprintln!("{:?}", e);
            ZapataResult::from(e)
        }
    }
}
