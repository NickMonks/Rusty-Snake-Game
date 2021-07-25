extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord_u32;

const BACK_COLOR: Color = [0.5,0.5,0.5,1.0];

fn main() {
    let (width, height) = (20,20);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord_u32(width), to_coord_u32(height)],
    ).exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new(width, height);

    // this method cleans up the window and resizes frame buffers
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // once there's been a callback from an event, we check which key it has been
            // and pass it to our game object
            game.key_pressed(key);
        }
        // pass a closure/function pointer - it will be called internally with the context and G2d buffer
        // and we pass those arguments to our draw game object
        window.draw_2d(&event, |c,g, _device| {
            clear(BACK_COLOR, g);
            game.draw(&c,g);
        });

        event.update(|args| {
            game.update(args.dt);
        });

    }
}
