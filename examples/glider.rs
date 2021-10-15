use game_of_life::World;
use std::{thread::sleep, time::Duration};

fn main() {
    let world = World::grid([[1, 1, 1], [0, 0, 1], [0, 1, 0]]);
    println!("{}", world);
    for iteration in world {
        sleep(Duration::from_millis(250));
        println!("{}", iteration);
    }
}
