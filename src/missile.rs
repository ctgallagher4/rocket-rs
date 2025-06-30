use std::time::Duration;

use crate::drawable::Drawable;
use crate::player::Player;
use crate::{MISSILE_SPEED, SHIP_SIZE};
use sdl3::pixels::Color;
use sdl3::render::{Canvas, FPoint};
use sdl3::video::Window;

/// A struct for controlling missiles
pub struct Missile {
    pub x: f32,
    pub y: f32,
    x_dot: f32,
    y_dot: f32,
    pub bearing: f32,
}

impl Missile {
    /// Creates a new missile.
    pub fn new(player: &Player) -> Self {
        let missile_add_x = player.bearing.cos() * MISSILE_SPEED;
        let missile_add_y = player.bearing.sin() * MISSILE_SPEED;
        Self {
            x: player.x,
            y: player.y,
            x_dot: player.x_dot + missile_add_x,
            y_dot: player.y_dot + missile_add_y,
            bearing: player.bearing,
        }
    }

    /// Updates a missile based on its velocity
    pub fn update(&mut self, delta: &Duration) {
        self.x += self.x_dot * delta.as_secs_f32();
        self.y += self.y_dot * delta.as_secs_f32();
    }
}

impl Drawable for Missile {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let x_top = self.x + SHIP_SIZE / 2.0 * self.bearing.cos();
        let y_top = self.y + SHIP_SIZE / 2.0 * self.bearing.sin();
        let top = FPoint::new(x_top, y_top);
        let bottom = FPoint::new(self.x, self.y);
        let color = canvas.draw_color();
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_line(top, bottom).expect("No driver failure");
        canvas.set_draw_color(color);
    }
}
