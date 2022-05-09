mod entity;
mod error;
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
    let mut players: Vec<Human> = Vec::with_capacity(DEFAULT_AGENT_COUNT);

    for i in 0..DEFAULT_AGENT_COUNT {
        let id = i as u64;
        players.push(Human::new(id, "Player".to_string(), 100));
    }

    println!("{:?}\n\nAdding...\n\n", players);

    let mut running = false;
    let mut tick = 0;
    match world.add_entity(players) {
        Ok(world) => {
            println!("starting world...");
            running = true;
            while running {
                if tick == 1000 {
                    running = false;
                }

                match world.tick() {
                    Some(e) => panic!("Tick #{} failed - {}", tick, e),
                    None => {
                        tick += 1;
                    }
                }
            }
            println!("done. ran for {} ticks", tick);
        }
        Err(e) => {
            panic!("Failed to add players: {}", e)
        }
    }
}
