use crate::ball::Ball;
use crate::paddle::{Paddle, PADDLE_SPEED};
use piston_window::{
    clear, rectangle, Button, ButtonEvent, ButtonState, Event, Key, PistonWindow, RenderEvent,
};

const BACKGROUND_COLOR: [f32; 4] = [0.0; 4];
const FOREGROUND_COLOR: [f32; 4] = [1.0; 4];

pub struct Game {
    window: PistonWindow,
    paddles: [Paddle; 2],
    ball: Ball,
}

impl Game {
    pub fn new(window: PistonWindow) -> Self {
        Game {
            window,
            paddles: [Paddle::new_left(), Paddle::new_right()],
            ball: Ball::new(),
        }
    }

    pub fn start(&mut self) {
        while let Some(e) = self.window.next() {
            if let Some(_) = e.render_args() {
                self.render(&e)
            }
            if let Some(b) = e.button_args() {
                if let Button::Keyboard(k) = b.button {
                    match b.state {
                        ButtonState::Press => self.key_down(k),
                        ButtonState::Release => self.key_up(k),
                    }
                }
            }
        }
    }

    fn render(&mut self, event: &Event) {
        self.paddles[0].update_position();
        let paddles = self.paddles.clone();
        let ball = self.ball.clone();
        self.window.draw_2d(event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            paddles
                .iter()
                .for_each(|p| rectangle(FOREGROUND_COLOR, p.transform, c.transform, g));
            rectangle(FOREGROUND_COLOR, ball.transform, c.transform, g);
        });
    }

    fn key_down(&mut self, key: Key) {
        if key == Key::A {
            self.paddles[0].movement[0] = PADDLE_SPEED
        }
        if key == Key::D {
            self.paddles[0].movement[1] = PADDLE_SPEED
        }
    }

    fn key_up(&mut self, key: Key) {
        if key == Key::A {
            self.paddles[0].movement[0] = 0.0
        }
        if key == Key::D {
            self.paddles[0].movement[1] = 0.0
        }
    }
}
