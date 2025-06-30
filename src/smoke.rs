use crate::{SMOKE_SIZE, drawable::Drawable};
use rand::Rng;
use sdl3::{
    pixels::Color,
    render::{Canvas, FPoint},
    video::Window,
};
use std::f32::consts::PI;

/// A struct for handling smoke
pub struct Smoke {
    x: f32,
    y: f32,
    pub frame: u32,
}

impl Smoke {
    /// Creates a new smoke object
    pub fn new(x: f32, y: f32) -> Smoke {
        Smoke { x, y, frame: 1 }
    }

    /// Advances the frame count
    pub fn update(&mut self) {
        self.frame += 1;
    }
}

impl Drawable for Smoke {
    /// A method to draw smoke
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let mut points: Vec<FPoint> = Vec::with_capacity(360);
        let mut rng = rand::rng();
        for mut angle in 0..=45 {
            angle *= 8;
            let rads = angle as f32 / 360.0 * 2.0 * PI;
            let radius = rng.random_range(0.0..1.0_f32).sqrt() * SMOKE_SIZE * self.frame as f32;
            let x = self.x + radius * rads.cos();
            let y = self.y + radius * rads.sin();
            points.push(FPoint::new(x, y));
        }
        let grayscale = rng.random_range(25..255_u8);
        let color = canvas.draw_color();
        canvas.set_draw_color(Color::RGB(grayscale, grayscale, grayscale));
        canvas.draw_points(&points[..]).expect("No driver failure");
        canvas.set_draw_color(color);
    }
}
