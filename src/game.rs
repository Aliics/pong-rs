use crate::paddle::{PADDLE_SPEED, PADDLE_START};
use piston_window::{
    clear, rectangle, Button, ButtonEvent, ButtonState, Event, Key, PistonWindow, RenderEvent,
};

const BACKGROUND_COLOR: [f32; 4] = [0.0; 4];
const FOREGROUND_COLOR: [f32; 4] = [1.0; 4];

pub struct Game {
    window: PistonWindow,
    paddle_transforms: [[f64; 4]; 2],
    paddle_movements: [[f64; 2]; 2],
}

impl Game {
    pub fn new(window: PistonWindow) -> Self {
        Game {
            window,
            paddle_transforms: PADDLE_START,
            paddle_movements: [[0.0; 2]; 2],
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

    fn calc_position(&mut self) {
        self.paddle_transforms[0][1] -= self.paddle_movements[0][0];
        self.paddle_transforms[0][1] += self.paddle_movements[0][1];
    }

    fn render(&mut self, event: &Event) {
        self.calc_position();
        let paddle_slices = self.paddle_transforms.clone();
        self.window.draw_2d(event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            paddle_slices
                .iter()
                .for_each(|p| rectangle(FOREGROUND_COLOR, *p, c.transform, g));
        });
    }

    fn key_down(&mut self, key: Key) {
        if key == Key::A {
            self.paddle_movements[0][0] = PADDLE_SPEED
        }
        if key == Key::D {
            self.paddle_movements[0][1] = PADDLE_SPEED
        }
    }

    fn key_up(&mut self, key: Key) {
        if key == Key::A {
            self.paddle_movements[0][0] = 0.0
        }
        if key == Key::D {
            self.paddle_movements[0][1] = 0.0
        }
    }
}
