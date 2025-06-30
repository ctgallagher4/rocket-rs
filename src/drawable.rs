use sdl3::{render::Canvas, video::Window};

/// A trait for drawable game objects.
pub trait Drawable {
    fn draw(&mut self, canvas: &mut Canvas<Window>);
}
