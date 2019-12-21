use crate::paddle::PADDLE_START;
use piston_window::{
    clear, rectangle, Button, ButtonArgs, ButtonEvent, ButtonState, Event, PistonWindow,
    RenderEvent,
};

const BACKGROUND_COLOR: [f32; 4] = [0.0; 4];
const FOREGROUND_COLOR: [f32; 4] = [1.0; 4];

pub struct Game {
    window: PistonWindow,
    paddle_transforms: [[f64; 4]; 2],
}

impl Game {
    pub fn new(window: PistonWindow) -> Self {
        Game {
            window,
            paddle_transforms: PADDLE_START,
        }
    }

    pub fn start(&mut self) {
        while let Some(e) = self.window.next() {
            if let Some(_) = e.render_args() {
                self.render(&e)
            }
            if let Some(b) = e.button_args() {
                match b.state {
                    ButtonState::Press => self.key_down(b.button),
                    ButtonState::Release => self.key_up(b.button),
                }
            }
        }
    }

    fn render(&mut self, event: &Event) {
        let paddle_slices = self.paddle_transforms.clone();
        self.window.draw_2d(event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            paddle_slices
                .iter()
                .for_each(|p| rectangle(FOREGROUND_COLOR, *p, c.transform, g));
        });
    }

    fn key_down(&mut self, button: Button) {
        println!("button down {:?}", button);
    }

    fn key_up(&mut self, button: Button) {
        println!("button up {:?}", button);
    }
}
