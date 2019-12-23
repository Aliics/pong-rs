use crate::window::WINDOW_SIZE;

const PADDLE_SIZE: [f64; 2] = [10.0, 100.0];

pub const PADDLE_SPEED: f64 = 5.0;

pub const PADDLE_LEFT_START: [f64; 4] = [0.0, 0.0, PADDLE_SIZE[0], PADDLE_SIZE[1]];
pub const PADDLE_RIGHT_START: [f64; 4] = [
    WINDOW_SIZE.0 as f64 - PADDLE_SIZE[0],
    0.0,
    PADDLE_SIZE[0],
    PADDLE_SIZE[1],
];

#[derive(Clone)]
pub struct Paddle {
    pub transform: [f64; 4],
    pub movement: [f64; 2],
}

impl Paddle {
    pub fn new_left() -> Self {
        Paddle {
            transform: PADDLE_LEFT_START,
            movement: [0.0; 2],
        }
    }

    pub fn new_right() -> Self {
        Paddle {
            transform: PADDLE_RIGHT_START,
            movement: [0.0; 2],
        }
    }

    pub fn update_position(&mut self) {
        self.transform[1] -= if self.transform[1] > 0.0 {
            self.movement[0]
        } else {
            0.0
        };
        self.transform[1] += if self.transform[1] < WINDOW_SIZE.1 as f64 - PADDLE_SIZE[1] {
            self.movement[1]
        } else {
            0.0
        };
    }
}
