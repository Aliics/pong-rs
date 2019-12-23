use crate::render::{window::WINDOW_SIZE, Renderable};

const BALL_SIZE: [f64; 2] = [10.0, 10.0];

const BALL_START: [f64; 2] = [WINDOW_SIZE.0 as f64 / 2.0, WINDOW_SIZE.1 as f64 / 2.0];

#[derive(Clone)]
pub struct Ball {
    pub transform: [f64; 4],
    movement: [f64; 2],
}

impl Ball {
    pub fn new() -> Self {
        let mut ball = Ball {
            transform: [0.0; 4],
            movement: [0.0; 2],
        };
        ball.reset();
        ball
    }

    pub fn reset(&mut self) {
        self.transform = [BALL_START[0], BALL_START[1], BALL_SIZE[0], BALL_SIZE[1]];
        self.movement = [0.0; 2];
    }
}

impl Renderable for Ball {
    fn transform(&self) -> [f64; 4] {
        self.transform
    }
}
