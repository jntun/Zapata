mod scene;
mod entity;
mod error;
mod physics;

use scene::Scene;
use crate::error::ZapataResult;

const EPOCH_COUNT: usize = 100;
const HUMAN_COUNT: usize = 10;



fn main() -> error::ZapataResult {
    let ref mut scene = Scene::new(None);

    match scene.run(EPOCH_COUNT) {
        Ok(()) => {
            print!("\nDone - Ran for {} ticks:\n\t", EPOCH_COUNT);
            println!("{:?}", scene);
            ZapataResult::Success
        },
        Err(e) => ZapataResult::from(e),
    }
}
