use crate::paddle::PADDLE_START;
use piston_window::{clear, rectangle, Event, PistonWindow};

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
            self.render(&e)
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
}
