use game_of_life::{Position, World};

const PIXELS_PER_CELL: f64 = 4.0;
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 640;
const STEP_SIZE: i32 = 5 * PIXELS_PER_CELL as i32;

extern crate piston_window;
use piston_window::*;
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Glider-Gun", (WINDOW_WIDTH, WINDOW_HEIGHT))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let mut world = World::grid([
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 1,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 1,
        ],
        [
            1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ],
        [
            1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ],
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ],
    ])
    .into_iter();

    let mut left = -(WINDOW_WIDTH as i32 / PIXELS_PER_CELL as i32) / 2;
    let mut bottom = -(WINDOW_HEIGHT as i32 / PIXELS_PER_CELL as i32) / 2;
    while let Some(e) = window.next() {
        let step = world.next();

        if let Some(button) = e.press_args() {
            left += match button {
                Button::Keyboard(Key::Left | Key::A) => -STEP_SIZE,
                Button::Keyboard(Key::Right | Key::D) => STEP_SIZE,
                _ => 0,
            };

            bottom += match button {
                Button::Keyboard(Key::Down | Key::S) => -STEP_SIZE,
                Button::Keyboard(Key::Up | Key::W) => STEP_SIZE,
                _ => 0,
            };
        }

        window.draw_2d(&e, move |c, g, _| {
            if let Some(world) = step {
                clear([1.0, 1.0, 1.0, 1.0], g);
                for Position(x, y) in world.cells.keys() {
                    rectangle(
                        [0.0, 0.0, 0.0, 1.0],
                        [
                            (x - left) as f64 * PIXELS_PER_CELL,
                            WINDOW_HEIGHT as f64 - (y - bottom) as f64 * PIXELS_PER_CELL,
                            PIXELS_PER_CELL,
                            PIXELS_PER_CELL,
                        ],
                        c.transform,
                        g,
                    );
                }
            }
        });
    }
}
