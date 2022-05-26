mod scene;
mod entity;
mod error;
mod physics;

use scene::Scene;

const EPOCH_COUNT: usize = 100;
const HUMAN_COUNT: usize = 10;



fn main() -> error::ZapataResult {
    let scene = Scene::new(None);

    print!("\nDone - Ran for {} ticks:\n\t", EPOCH_COUNT);
    println!("{:?}", scene);
    self::error::ZapataResult::Success
}
