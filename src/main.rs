mod entity;
mod human;
mod world;

use entity::Entity;
use human::Human;
use world::World;

const DEFAULT_MAX_ENTITIES: usize = 128;
const DEFAULT_AGENT_COUNT: usize = 11;
const DEFAULT_TEAM_COUNT: usize = 2;

fn main() {
    let mut world = World::new(
        "Zapata",
        DEFAULT_MAX_ENTITIES,
        DEFAULT_TEAM_COUNT,
        DEFAULT_AGENT_COUNT,
    );
    let mut players: Vec<Box<dyn Entity>> = Vec::with_capacity(DEFAULT_AGENT_COUNT);

    for i in 0..DEFAULT_AGENT_COUNT {
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
