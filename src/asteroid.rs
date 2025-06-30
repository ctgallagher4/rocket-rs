use crate::{
    ASTEROID_NUM_POINTS, ASTEROID_PERT_SIZE, ASTEROID_SIZE_MAX, ASTEROID_SIZE_MIN, SPEED_LIMIT,
    WIDTH,
    drawable::Drawable,
    game::{BoundsChecked, GameObject},
};
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};
use sdl3::{
    render::{Canvas, FPoint},
    video::Window,
};
use std::{f32::consts::PI, time::Duration};

/// An enum to control what side the asteroids start on
enum SpawnSide {
    Top,
    Bottom,
    Left,
    Right,
}

impl Distribution<SpawnSide> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SpawnSide {
        match rng.random_range(0..4) {
            0 => SpawnSide::Top,
            1 => SpawnSide::Bottom,
            2 => SpawnSide::Left,
            _ => SpawnSide::Right,
        }
    }
}

/// A struct for managing asteroids.
pub struct Asteroid {
    x: f32,
    y: f32,
    pub rad: f32,
    random_pert: Vec<f32>,
    x_dot: f32,
    y_dot: f32,
}

impl Asteroid {
    /// A method for creating new asteroids.
    pub fn new() -> Self {
        let spawn_side: SpawnSide = rand::random();
        let coord = rand::random_range(0.0..=(WIDTH as f32));
        let spawn_coords = match spawn_side {
            SpawnSide::Top => (coord, 0.0),
            SpawnSide::Bottom => (coord, WIDTH as f32),
            SpawnSide::Left => (0.0, coord),
            SpawnSide::Right => (WIDTH as f32, coord),
        };
        let rad = rand::random_range(ASTEROID_SIZE_MIN..=ASTEROID_SIZE_MAX);
        let vel = || rand::random_range(-1.0 * SPEED_LIMIT / 2.0..SPEED_LIMIT / 2.0);
        let x_dot = vel();
        let y_dot = vel();

        let mut random_pert: Vec<f32> = Vec::new();
        (0..ASTEROID_NUM_POINTS).for_each(|_| {
            random_pert.push(rand::random_range(-ASTEROID_PERT_SIZE..ASTEROID_PERT_SIZE));
        });

        Self {
            x: spawn_coords.0,
            y: spawn_coords.1,
            rad,
            random_pert,
            x_dot,
            y_dot,
        }
    }

    /// A method to update an asteroid.
    pub fn update(&mut self, delta: &Duration) {
        let pos_update = |pos: &mut f32, vel: &mut f32| *pos += *vel * delta.as_secs_f32();
        pos_update(&mut self.x, &mut self.x_dot);
        pos_update(&mut self.y, &mut self.y_dot);
        self.bounds_check_update();
    }
}

impl Drawable for Asteroid {
    fn draw(&mut self, canvas: &mut Canvas<Window>) {
        let length = self.random_pert.len();
        let mut lines: Vec<FPoint> = Vec::with_capacity(length - 1);
        for i in 0..(length - 1) {
            let calc_angle = |i| i as f32 * 2.0 * PI / length as f32;
            let angle = calc_angle(i);
            let create_point = |angle: f32| {
                FPoint::new(
                    self.x + (self.rad + self.random_pert[i]) * angle.cos(),
                    self.y + self.rad * angle.sin(),
                )
            };
            let point = create_point(angle);
            lines.push(point);
        }
        lines.push(lines[0]);
        canvas.draw_lines(&lines[..]).expect("No driver Errors");
    }
}

impl GameObject for Asteroid {
    fn get_x(&self) -> f32 {
        self.x
    }

    fn get_y(&self) -> f32 {
        self.y
    }

    fn get_x_dot(&self) -> f32 {
        self.x_dot
    }

    fn get_y_dot(&self) -> f32 {
        self.y_dot
    }

    fn set_x(&mut self, val: f32) {
        self.x = val;
    }

    fn set_x_dot(&mut self, val: f32) {
        self.x_dot = val;
    }

    fn set_y(&mut self, val: f32) {
        self.y = val;
    }

    fn set_y_dot(&mut self, val: f32) {
        self.y_dot = val;
    }
}
