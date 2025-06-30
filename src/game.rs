use crate::asteroid::Asteroid;
use crate::drawable::Drawable;
use crate::missile::Missile;
use crate::player::{Direction, Player};
use crate::smoke::Smoke;
use crate::{MISSILE_FIRE_EVERY, SHIP_SIZE, SMOKE_FRAMES, WIDTH};
use sdl3::keyboard::{KeyboardState, Scancode};
use sdl3::render::Canvas;
use sdl3::video::Window;
use std::f32::consts::PI;
use std::time::Duration;

/// A trait for manipulating object positions.
pub trait GameObject {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_x_dot(&self) -> f32;
    fn get_y_dot(&self) -> f32;
    fn set_x(&mut self, val: f32);
    fn set_y(&mut self, val: f32);
    fn set_x_dot(&mut self, val: f32);
    fn set_y_dot(&mut self, val: f32);
}

impl<T> BoundsChecked for T where T: GameObject {}

pub trait BoundsChecked: GameObject {
    fn bounds_check_update(&mut self) {
        if self.get_x() > WIDTH as f32 {
            self.set_x(0.0);
        } else if self.get_x() < 0 as f32 {
            self.set_x(WIDTH as f32);
        }
        if self.get_y() > WIDTH as f32 {
            self.set_y(0.0);
        } else if self.get_y() < 0 as f32 {
            self.set_y(WIDTH as f32);
        }
    }
}

/// A struct for managing everything
pub struct Game {
    pub score: u32,
    player: Player,
    asteroids: Vec<Asteroid>,
    smoke: Vec<Smoke>,
    missiles: Vec<Missile>,
    missile_duration: Duration,
    time_since_last_missile: Duration,
}

impl Game {
    /// Creates a new game.
    pub fn new() -> Self {
        let player = Player::new(500.0, 500.0, 0.0, 0.0, 0.0);
        let asteroids = (0..10).map(|_| Asteroid::new()).collect();
        let score = 0;
        Self {
            score,
            player,
            asteroids,
            missiles: Vec::new(),
            missile_duration: Duration::from_millis(MISSILE_FIRE_EVERY),
            time_since_last_missile: Duration::from_millis(0),
            smoke: Vec::new(),
        }
    }

    /// Updates the game.
    pub fn update(
        &mut self,
        canvas: &mut Canvas<Window>,
        keyboard_state: &KeyboardState,
        delta: &Duration,
    ) -> bool {
        self.time_since_last_missile += *delta;
        let key_w = KeyboardState::is_scancode_pressed(&keyboard_state, Scancode::W);
        let key_a = KeyboardState::is_scancode_pressed(&keyboard_state, Scancode::A);
        let key_d = KeyboardState::is_scancode_pressed(&keyboard_state, Scancode::D);
        let key_space = KeyboardState::is_scancode_pressed(&keyboard_state, Scancode::Space);

        if key_w {
            self.player.forward(delta);
            let back_x = self.player.x + SHIP_SIZE * (self.player.bearing + PI).cos();
            let back_y = self.player.y + SHIP_SIZE * (self.player.bearing + PI).sin();
            self.smoke.push(Smoke::new(back_x, back_y));
        }
        if key_a {
            self.player.turn(delta, Direction::CounterClockwise);
        }
        if key_d {
            self.player.turn(delta, Direction::Clockwise);
        }
        if key_space && self.missile_duration < self.time_since_last_missile {
            let missile = Missile::new(&self.player);
            self.missiles.push(missile);
            self.time_since_last_missile = Duration::from_millis(0);
        }

        // update asteroids
        self.asteroids.iter_mut().for_each(|asteroid| {
            asteroid.update(delta);
            asteroid.draw(canvas);
        });

        // update player
        self.player.update(delta);
        self.player.draw(canvas);

        // update missiles
        let mut indices_to_drop = Vec::new();
        self.missiles
            .iter_mut()
            .enumerate()
            .for_each(|(i, missile)| {
                missile.update(delta);
                missile.draw(canvas);
                if missile.x > WIDTH as f32
                    || missile.x < 0.0
                    || missile.y > WIDTH as f32
                    || missile.y < 0.0
                {
                    indices_to_drop.push(i);
                }
            });
        let mut count: usize = 0;
        for i in indices_to_drop {
            self.missiles.remove(i - count);
            count += 1;
        }

        // check missile asteroid collision
        let mut asteroids_to_drop = Vec::new();
        self.missiles.iter().for_each(|missile| {
            self.asteroids
                .iter()
                .enumerate()
                .for_each(|(a_i, asteroid)| {
                    let x_sq = (missile.x - asteroid.get_x()).powf(2.0);
                    let y_sq = (missile.y - asteroid.get_y()).powf(2.0);
                    if (x_sq + y_sq).powf(0.5) < asteroid.rad + SHIP_SIZE / 2.0 {
                        asteroids_to_drop.push(a_i);
                        self.smoke
                            .push(Smoke::new(asteroid.get_x(), asteroid.get_y()));
                        self.score += 1;
                    }
                });
        });
        for (iter_count, index) in asteroids_to_drop.iter().enumerate() {
            self.asteroids.remove(index - iter_count);
        }

        // update exhaust
        let mut indices_to_drop = Vec::new();
        self.smoke
            .iter_mut()
            .enumerate()
            .for_each(|(index, smoke)| {
                smoke.update();
                smoke.draw(canvas);
                if smoke.frame > SMOKE_FRAMES {
                    indices_to_drop.push(index)
                }
            });
        let mut count: usize = 0;
        for i in indices_to_drop {
            self.smoke.remove(i - count);
            count += 1;
        }

        // check player asteroid collision
        let mut cont = true;
        self.asteroids.iter().for_each(|asteroid| {
            let x_sq = (self.player.x - asteroid.get_x()).powf(2.0);
            let y_sq = (self.player.y - asteroid.get_y()).powf(2.0);
            if (x_sq + y_sq).powf(0.5) < asteroid.rad + SHIP_SIZE / 2.0 {
                cont = false;
            }
        });

        cont
    }
}
