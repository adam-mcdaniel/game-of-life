use game_of_life::World;

fn main() {
    let world = World::grid([[1, 1, 0], [1, 0, 1], [0, 1, 0]]);
    println!("{}", world);
    for iteration in world {
        println!("{}", iteration);
    }
}
