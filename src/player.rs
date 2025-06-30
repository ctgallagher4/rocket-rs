use crate::drawable::Drawable;
use crate::game::{BoundsChecked, GameObject};
use crate::{ACCEL, SHIP_SIZE, SPEED_LIMIT, TURN_SPEED_LIMIT};
use sdl3::render::FPoint;
use sdl3::{render::Canvas, video::Window};
use std::f32::consts::PI;
use std::time::Duration;

/// Controls the triangle that the player manipulates
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub x_dot: f32,
    pub y_dot: f32,
    pub bearing: f32,
}

/// A direction enum for turning clockwise or counter clockwise.
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Player {
    /// Creates a new player.
    pub fn new(x: f32, y: f32, x_dot: f32, y_dot: f32, bearing: f32) -> Self {
        Self {
            x,
            y,
            x_dot,
            y_dot,
            bearing,
        }
    }

    /// Accelerates the player forward.
    pub fn forward(&mut self, delta: &Duration) {
        self.x_dot += ACCEL * self.bearing.cos() * delta.as_secs_f32();
        self.y_dot += ACCEL * self.bearing.sin() * delta.as_secs_f32();
        if self.x_dot > SPEED_LIMIT || self.x_dot < -SPEED_LIMIT {
            let ratio = SPEED_LIMIT / self.x_dot;
            self.x_dot = ratio * self.x_dot.abs();
        }
        if self.y_dot > SPEED_LIMIT || self.y_dot < -SPEED_LIMIT {
            let ratio = SPEED_LIMIT / self.y_dot;
            self.y_dot = ratio * self.y_dot.abs();
        }
    }

    /// Turns the player in a direction.
    pub fn turn(&mut self, delta: &Duration, direction: Direction) {
        let mag = TURN_SPEED_LIMIT * 2.0 * 3.14159 / 360.0 * delta.as_secs_f32() as f32;
        match direction {
            Direction::Clockwise => self.bearing += mag,
            Direction::CounterClockwise => self.bearing -= mag,
        }
    }

    /// Updates the player.
    pub fn update(&mut self, delta: &Duration) {
        self.x += self.x_dot * delta.as_secs_f32();
        self.y += self.y_dot * delta.as_secs_f32();
        self.bounds_check_update();
    }
}

impl GameObject for Player {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_x_dot(&self) -> f32 {
        self.x_dot
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn get_y_dot(&self) -> f32 {
        self.y_dot
    }

    fn set_x(&mut self, val: f32) {
        self.x = val;
    }

    fn set_y(&mut self, val: f32) {
        self.y = val;
    }

    fn set_x_dot(&mut self, val: f32) {
        self.x_dot = val;
    }

    fn set_y_dot(&mut self, val: f32) {
        self.y_dot = val;
    }
}

impl Drawable for Player {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let top_x = self.x + self.bearing.cos() * SHIP_SIZE;
        let top_y = self.y + self.bearing.sin() * SHIP_SIZE;

        let top = FPoint::new(top_x.round(), top_y.round());
        let bot_left = calculate_point(self.x, self.y, self.bearing, 200.0);
        let bot_right = calculate_point(self.x, self.y, self.bearing, 160.0);
        let points_array = [top, bot_left, bot_right, top];
        canvas
            .draw_lines(&points_array[..])
            .expect("No driver error!");
    }
}

/// A helper method to calculate the player's triangle.
fn calculate_point(x: f32, y: f32, bear: f32, deg: f32) -> FPoint {
    let (left_x, left_y) = (
        (x + (deg * 2.0 * PI / 360.0 + bear).cos() * SHIP_SIZE),
        (y + (deg * 2.0 * PI / 360.0 + bear).sin() * SHIP_SIZE),
    );
    FPoint::new(left_x, left_y)
}
