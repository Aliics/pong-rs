use crate::paddle::Paddle;
use crate::render::{window::WINDOW_SIZE, Renderable};

const BALL_SIZE: [f64; 2] = [10.0, 10.0];

const BALL_START: [f64; 2] = [WINDOW_SIZE.0 as f64 / 2.0, WINDOW_SIZE.1 as f64 / 2.0];

const BALL_SPEED: f64 = 5.0;

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

    pub fn update_position(&mut self, player_paddle: &Paddle, computer_paddle: &Paddle) {
        let (x, y) = (self.transform[0], self.transform[1]);
        if x > WINDOW_SIZE.0 as f64 || x < 0.0 {
            if (x > WINDOW_SIZE.0 as f64 && self.on_paddle(computer_paddle))
                || (x < 0.0 && self.on_paddle(player_paddle))
            {
                self.movement[0] *= -1.0;
            } else {
                self.reset();
            }
        }
        if y > WINDOW_SIZE.1 as f64 || y < 0.0 {
            self.movement[1] *= -1.0;
        }
        self.transform[0] += self.movement[0];
        self.transform[1] += self.movement[1];
    }

    fn on_paddle(&self, paddle: &Paddle) -> bool {
        self.transform[1] > paddle.transform[1]
            && self.transform[1] < paddle.transform[1] + paddle.transform[3]
    }

    fn reset(&mut self) {
        self.transform = [BALL_START[0], BALL_START[1], BALL_SIZE[0], BALL_SIZE[1]];
        self.movement = [-BALL_SPEED, 0.0];
    }
}

impl Renderable for Ball {
    fn transform(&self) -> [f64; 4] {
        self.transform
    }
}
