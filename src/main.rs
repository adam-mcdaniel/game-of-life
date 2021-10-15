use game_of_life::{Position, World};
use std::{str::FromStr, time::Duration, thread::sleep};

fn main() {
    // let mut cells = vec![];
    // for i in 0..2000 {
    //     cells.extend([
    //         Position(i * 6 + 0, 0),
    //         Position(i * 6 + 1, 0),
    //         Position(i * 6 + 2, 0),
    //         Position(i * 6 + 2, -1),
    //         Position(i * 6 + 1, -2),
    //     ])
    // }
    // let world = World::grid(cells);

    // let world = World::grid([
    //     [1, 1, 1],
    //     [0, 0, 1],
    //     [0, 1, 0],
    // ]);
    let world = World::random(150, 150, 0.3);
    println!("{}", world);
    
    for (i, step) in world.clone().enumerate() {
        step.draw(0, 0, 100, 50);
    }
}