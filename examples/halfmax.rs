use game_of_life::{Position, World};
// use std::str::FromStr;
use std::{str::FromStr, thread::sleep, time::Duration};

const PIXELS_PER_CELL: f64 = 2.0;
const STEP_SIZE: i32 = 5 * PIXELS_PER_CELL as i32;

extern crate piston_window;
use piston_window::*;
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Halfmax", (640, 640))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    println!("Use WASD or Arrow keys to move around the world!");

    let mut world = World::from_str(include_str!("halfmax.txt"))
        .unwrap()
        .into_iter();

    let mut left = -(520 / PIXELS_PER_CELL as i32) / 2;
    let mut top = 500 / PIXELS_PER_CELL as i32;
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
            // use std::{str::FromStr, thread::sleep, time::Duration};
            let step = world.next();
            let Size { width, height } = window.size();
            window.draw_2d(&e, move |c, g, _| {
                if let Some(world) = step {
                    if world.cells.len() < 20000 {
                        sleep(Duration::from_millis(10));
                    }
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
