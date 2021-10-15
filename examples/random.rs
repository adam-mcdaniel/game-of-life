use game_of_life::{Position, World};

const PIXELS_PER_CELL: f64 = 4.0;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 640;
const STEP_SIZE: i32 = 5 * PIXELS_PER_CELL as i32;

extern crate piston_window;
use piston_window::*;
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Random", (WINDOW_WIDTH, WINDOW_HEIGHT))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    println!("Use WASD or Arrow keys to move around the world!");
    let mut world = World::random(300, 300, 0.2).into_iter();

    let mut left = 0;
    let mut top = 200;
    while let Some(e) = window.next() {
        if let Some(button) = e.press_args() {
            left += match button {
                Button::Keyboard(Key::Left | Key::A) => -STEP_SIZE,
                Button::Keyboard(Key::Right | Key::D) => STEP_SIZE,
                _ => 0,
            };

            top += match button {
                Button::Keyboard(Key::Down | Key::S) => -STEP_SIZE,
                Button::Keyboard(Key::Up | Key::W) => STEP_SIZE,
                _ => 0,
            };
        } else {
            let step = world.next();
            let Size { width, height } = window.size();
            window.draw_2d(&e, move |c, g, _| {
                if let Some(world) = step {
                    println!("{}", world.cells.len());
                    clear([1.0, 1.0, 1.0, 1.0], g);

                    for y in 0..(height as f64 / PIXELS_PER_CELL) as i32 {
                        for x in left..(left + (width as f64 / PIXELS_PER_CELL) as i32) {
                            let pos = Position(x, top - y);
                            if world.is_alive(pos) {
                                rectangle(
                                    [0.0, 0.0, 0.0, 1.0],
                                    [
                                        (x - left) as f64 * PIXELS_PER_CELL,
                                        y as f64 * PIXELS_PER_CELL,
                                        PIXELS_PER_CELL,
                                        PIXELS_PER_CELL,
                                    ],
                                    c.transform,
                                    g,
                                );
                            }
                        }
                    }
                }
            });
        }
    }
}
