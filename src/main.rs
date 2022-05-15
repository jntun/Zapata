mod entity;
mod error;
mod physics;

use physics::vec3::Vec3;
use entity::human::Human;
use entity::health::*;
use crate::error::TickError;

fn main() {
    let ref mut p1 = Human::new(100, None, Some(String::from("player 1")), None);
    let ref mut p2 = Human::new(100, Some(50), Some(String::from("player 2")), Some(Vec3::new(10.0, 0.0, 0.0)));

    println!("{} | {}\n{}", p1, p2, "-".repeat(50));
    p1.attack(p2);
    println!("{}", "-".repeat(50));
    println!("{:?}\n\n{:?}", p1, p2);
}
