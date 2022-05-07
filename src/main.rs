mod entity;
mod human;
mod world;

use entity::Entity;
use human::Human;
use world::World;

fn main() {
    let mut justin = Human::new("Justin", 100);
    println!("{:?}", justin);
    let mut world = World::new("Zapata");
    println!("{:?}", world);
    match world.add_entity(Box::new(&justin)) {
        Ok(world) => {
            println!("{:?}", world);
        }
        Err(e) => {
            panic!("Failed to add human {:?}:\n\t{} ", justin), e;
        }
    }
}
