use sdl2::pixels::Color;

pub trait Renderer {
    fn draw_text(&mut self, text: String, x: i32, y: i32, color: &Color);
    fn draw_rect(&mut self, center_x: i32, center_y: i32, width: u32, height: u32, color: &Color);
}
