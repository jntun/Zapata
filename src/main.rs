mod world;
mod entity;
mod error;
mod physics;

use std::process::exit;
use world::World;
use entity::human::Human;

const EPOCH_COUNT: usize = 100;
const HUMAN_COUNT: usize = 10;

fn main() {
    let ref mut world = World::new(None);
    let humans: Vec<_> = (0..HUMAN_COUNT).into_iter().map(|_| world::tracked::Entity::Physics(Box::new(Human::default()))).collect();

    for human in humans.into_iter() {
        world.add_entity(human);
    }

    println!("Running {}x...\n\t{:?}\n", EPOCH_COUNT, world);

    for i in 0..EPOCH_COUNT {
        match world.tick() {
            Ok(()) => println!("tick {}", i),
            Err(e) => {
                eprintln!("Couldn't tick world: {}", e);
                exit(1)
            },
        }
    }

    println!("\nDone:\n\t{:?}", world);
    exit(0)
}
