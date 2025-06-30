use rocket_rs::WIDTH;
use rocket_rs::game::Game;
use sdl3::event::Event;
use sdl3::keyboard::{KeyboardState, Keycode};
use sdl3::pixels::Color;
use std::time::{Duration, Instant};

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rocket", WIDTH, WIDTH)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();
    let mut game = Game::new();
    let mut instant = Instant::now();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let keyboard_state = KeyboardState::new(&event_pump);
        let delta = instant.elapsed();
        instant = Instant::now();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        if !game.update(&mut canvas, &keyboard_state, &delta) {
            println!("Your score is {}", game.score);
            break 'running;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
