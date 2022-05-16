mod world;
mod entity;
mod error;
mod physics;

use std::cell::RefCell;
use std::rc::Rc;
use world::World;
use physics::vec3::Vec3;
use entity::human::Human;
use entity::health::*;
use crate::entity::Entity;


fn main() {
    let p1 = Human::new(100, None, Some(String::from("player 1")), None);
    let p2 = Human::new(100, Some(50), Some(String::from("player 2")), Some(Vec3::new(10.0, 0.0, 0.0)));
    let ref mut world = World::new(None);
    world.add_entity(Rc::new(RefCell::new(Box::new(p1))));
    world.add_entity(Rc::new(RefCell::new(Box::new(p2))));

    println!("{:?}\n{}", world, "-".repeat(50));
    world.tick();
}
