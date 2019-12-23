use crate::{
    ball::Ball,
    paddle::{Paddle, PADDLE_SPEED},
    render::{Renderable, BACKGROUND_COLOR},
};
use piston_window::{
    clear, Button, ButtonEvent, ButtonState, Event, Key, PistonWindow, RenderEvent,
};

pub struct Game {
    window: PistonWindow,
    player_paddle: Paddle,
    computer_paddle: Paddle,
    ball: Ball,
}

impl Game {
    pub fn new(window: PistonWindow) -> Self {
        Game {
            window,
            player_paddle: Paddle::new_left(),
            computer_paddle: Paddle::new_right(),
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
        self.player_paddle.update_position();
        self.computer_paddle.update_position();
        self.ball
            .update_position(&self.player_paddle, &self.computer_paddle);
        let player_paddle = self.player_paddle.clone();
        let computer_paddle = self.computer_paddle.clone();
        let ball = self.ball.clone();
        self.window.draw_2d(event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            player_paddle.render(c, g);
            computer_paddle.render(c, g);
            ball.render(c, g);
        });
    }

    fn key_down(&mut self, key: Key) {
        if key == Key::A {
            self.player_paddle.movement[0] = PADDLE_SPEED
        }
        if key == Key::D {
            self.player_paddle.movement[1] = PADDLE_SPEED
        }
    }

    fn key_up(&mut self, key: Key) {
        if key == Key::A {
            self.player_paddle.movement[0] = 0.0
        }
        if key == Key::D {
            self.player_paddle.movement[1] = 0.0
        }
    }
}
