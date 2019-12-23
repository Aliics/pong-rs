use crate::game::Game;
use crate::window::{WINDOW_NAME, WINDOW_SIZE};
use piston_window::{PistonWindow, WindowSettings};

mod ball;
mod game;
mod paddle;
mod window;

fn main() {
    let window: PistonWindow = WindowSettings::new(WINDOW_NAME, WINDOW_SIZE)
        .build()
        .unwrap();
    Game::new(window).start();
}
