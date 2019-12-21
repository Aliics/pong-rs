use piston_window::{clear, rectangle, Event, PistonWindow, WindowSettings};

const WINDOW_NAME: &str = "pong-rs";
const DEFAULT_WINDOW_SIZE: (u32, u32) = (648, 480);

const BACKGROUND_COLOR: [f32; 4] = [0.0; 4];
const FOREGROUND_COLOR: [f32; 4] = [1.0; 4];
const PADDLE_TRANSFORM: [f64; 4] = [0.0, 0.0, 10.0, 100.0];

struct Game {
    window: PistonWindow,
}

impl Game {
    fn new(window: PistonWindow) -> Self {
        Game { window }
    }

    fn render(&mut self, event: &Event) {
        self.window.draw_2d(event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            rectangle(FOREGROUND_COLOR, PADDLE_TRANSFORM, c.transform, g);
        });
    }

    fn start(&mut self) {
        while let Some(e) = self.window.next() {
            self.render(&e)
        }
    }
}

fn main() {
    let window: PistonWindow = WindowSettings::new(WINDOW_NAME, DEFAULT_WINDOW_SIZE)
        .build()
        .unwrap();
    Game::new(window).start();
}
