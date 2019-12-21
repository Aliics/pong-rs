use piston_window::{clear, rectangle, Event, PistonWindow, WindowSettings};

const WINDOW_NAME: &str = "pong-rs";
const WINDOW_SIZE: (u32, u32) = (648, 480);

const BACKGROUND_COLOR: [f32; 4] = [0.0; 4];
const FOREGROUND_COLOR: [f32; 4] = [1.0; 4];

const PADDLE_SIZE: [f64; 2] = [10.0, 100.0];

struct Game {
    window: PistonWindow,
    paddle_transforms: [[f64; 4]; 2],
}

impl Game {
    fn new(window: PistonWindow) -> Self {
        Game {
            window,
            paddle_transforms: [
                [0.0, 0.0, PADDLE_SIZE[0], PADDLE_SIZE[1]],
                [
                    WINDOW_SIZE.0 as f64 - PADDLE_SIZE[0],
                    0.0,
                    PADDLE_SIZE[0],
                    PADDLE_SIZE[1],
                ],
            ],
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

    fn start(&mut self) {
        while let Some(e) = self.window.next() {
            self.render(&e)
        }
    }
}

fn main() {
    let window: PistonWindow = WindowSettings::new(WINDOW_NAME, WINDOW_SIZE)
        .build()
        .unwrap();
    Game::new(window).start();
}
