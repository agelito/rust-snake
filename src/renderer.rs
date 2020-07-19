use sdl2::pixels::Color;

pub trait Renderer {
    fn push_rect(&mut self, center_x: i32, center_y: i32, width: u32, height: u32, color: &Color);
}
