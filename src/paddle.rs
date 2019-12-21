use crate::window::WINDOW_SIZE;

const PADDLE_SIZE: [f64; 2] = [10.0, 100.0];

pub const PADDLE_START: [[f64; 4]; 2] = [
    [0.0, 0.0, PADDLE_SIZE[0], PADDLE_SIZE[1]],
    [
        WINDOW_SIZE.0 as f64 - PADDLE_SIZE[0],
        0.0,
        PADDLE_SIZE[0],
        PADDLE_SIZE[1],
    ],
];
