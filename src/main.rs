mod entity;
mod error;

use entity::human::Human;
use entity::health::*;

fn main() {
    let ref mut p1 = Human::new(100, None, Some(String::from("player 1")));
    let ref mut p2 = Human::new(100, Some(50), Some(String::from("player 2")));

    println!("p1: {} | p2: {}", p1, p2);
    p1.attack(p2);
    println!("p1: {} | p2: {}", p1, p2);
}
