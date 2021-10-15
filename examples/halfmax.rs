use game_of_life::{Position, World};
use std::str::FromStr;

const PIXELS_PER_CELL: f64 = 4.0;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 640;

extern crate piston_window;
use piston_window::*;
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Halfmax", (WINDOW_WIDTH, WINDOW_HEIGHT))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let mut world = World::from_str(include_str!("halfmax.txt"))
        .unwrap()
        .into_iter();

    while let Some(e) = window.next() {
        let step = world.next();

        window.draw_2d(&e, move |c, g, _| {
            if let Some(world) = step {
                clear([1.0, 1.0, 1.0, 1.0], g);
                let (l, _, w) = world.width();
                let (b, _, h) = world.height();
                for (pos, _) in world.cells {
                    let Position(x, y) = pos;
                    let left =
                        l + w as i32 / 2 - (WINDOW_WIDTH as f64 / PIXELS_PER_CELL) as i32 / 2;
                    let bottom =
                        b + h as i32 / 2 - (WINDOW_HEIGHT as f64 / PIXELS_PER_CELL) as i32 / 2;
                    rectangle(
                        [0.0, 0.0, 0.0, 1.0],
                        [
                            (x - left) as f64 * PIXELS_PER_CELL,
                            WINDOW_HEIGHT as f64 - (y - bottom) as f64 * PIXELS_PER_CELL,
                            PIXELS_PER_CELL,
                            PIXELS_PER_CELL,
                        ], // rectangle
                        c.transform,
                        g,
                    );
                }
            }
        });
    }
}
