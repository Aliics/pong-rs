use piston_window::{Context, G2d};

pub mod window;

pub const BACKGROUND_COLOR: [f32; 4] = [0.0; 4];
pub const FOREGROUND_COLOR: [f32; 4] = [1.0; 4];

pub trait Renderable {
    fn render(&self, context: Context, graphics: &mut G2d);
}
