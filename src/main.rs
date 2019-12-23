use crate::{
    game::Game,
    render::window::{WINDOW_NAME, WINDOW_SIZE},
};
use piston_window::{PistonWindow, WindowSettings};

mod ball;
mod game;
mod paddle;
mod render;

fn main() {
    let window: PistonWindow = WindowSettings::new(WINDOW_NAME, WINDOW_SIZE)
        .build()
        .unwrap();
    Game::new(window).start();
}
