mod asteroid;
mod drawable;
pub mod game;
mod missile;
mod player;
pub mod smoke;

pub const WIDTH: u32 = 1000;
pub const SMOKE_FRAMES: u32 = 50;
const ACCEL: f32 = 70.0;
const SPEED_LIMIT: f32 = 100.0;
const TURN_SPEED_LIMIT: f32 = 150.0;
const SHIP_SIZE: f32 = 15.0;
const MISSILE_SPEED: f32 = 300.0;
const MISSILE_FIRE_EVERY: u64 = 300; // ms
const SMOKE_SIZE: f32 = 1.0;
const ASTEROID_SIZE_MIN: f32 = 20.0;
const ASTEROID_SIZE_MAX: f32 = 50.0;
const ASTEROID_NUM_POINTS: u32 = 20;
const ASTEROID_PERT_SIZE: f32 = 5.0;
