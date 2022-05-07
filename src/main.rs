mod entity;
mod human;
mod world;

use entity::Entity;
use human::Human;
use world::World;

const MAX_ENTITIES: u64 = 128;
const AGENT_COUNT: usize = 11;
const TEAM_COUNT: usize = 2;
const TEAM_SIZE: usize = AGENT_COUNT / TEAM_COUNT;

fn main() {
    let mut world = World::new("Zapata");
    let mut players: Vec<Box<dyn Entity>> = Vec::with_capacity(AGENT_COUNT);

    for i in 0..AGENT_COUNT {
        players.push(Box::new(
            Human::new(format!("Player #{}", i).as_str(), 100).clone(),
        ));
    }

    println!("{:?}\n\nAdding...\n\n", players);

    match world.add_entity(&players) {
        Ok(world) => {
            println!("{:?}", world);
        }
        Err(e) => {
            panic!("Failed to add players: {}", e)
        }
    }
}
