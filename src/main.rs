mod entity;
mod error;

use entity::human::Human;
use entity::health::Health;

fn main() {
    let mut p1 = Human::new(100, None, Some(String::from("player 1")));
    let mut p2 = Human::new(100, Some(50), Some(String::from("player 2")));

    println!("p1: {} | p2: {}", p1, p2);
    p2.do_damage(2);
    println!("p1: {} | p2: {}", p1, p2);
}
