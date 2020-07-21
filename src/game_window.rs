use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureQuery, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::EventPump;

use crate::renderer::Renderer;

pub struct GameWindow {
    ttf: Sdl2TtfContext,
    canvas: WindowCanvas,
    event_pump: EventPump,
    screen_center_x: i32,
    screen_center_y: i32,
    pressed_keys: Vec<Keycode>,
}

pub enum GameWindowState {
    Open,
    Closed,
}

impl GameWindow {
    pub fn create(title: &str, initial_size: (u32, u32)) -> GameWindow {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();

        let window = video
            .window(title, initial_size.0, initial_size.1)
            .position_centered()
            .resizable()
            .allow_highdpi()
            .build()
            .unwrap();
        let ttf = sdl2::ttf::init().unwrap();

        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        let event_pump = context.event_pump().unwrap();

        let center_x = (initial_size.0 / 2) as i32;
        let center_y = (initial_size.1 / 2) as i32;
        GameWindow {
            ttf,
            canvas,
            event_pump,
            screen_center_x: center_x,
            screen_center_y: center_y,
            pressed_keys: Vec::new(),
        }
    }

    pub fn event_loop(&mut self) -> GameWindowState {
        self.pressed_keys.clear();
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return GameWindowState::Closed,
                Event::KeyDown { keycode, .. } => self.pressed_keys.push(keycode.unwrap()),
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(x, y) = win_event {
                        let center_x = (x / 2) as i32;
                        let center_y = (y / 2) as i32;

                        self.screen_center_x = center_x;
                        self.screen_center_y = center_y;
                    }
                }
                _ => {}
            }
        }

        return GameWindowState::Open;
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn redraw(&mut self) {
        self.canvas.present();
    }

    pub fn pressed_keys(&mut self) -> &Vec<Keycode> {
        &self.pressed_keys
    }
}

impl Renderer for GameWindow {
    fn draw_rect(&mut self, center_x: i32, center_y: i32, width: u32, height: u32, color: &Color) {
        let center = Point::new(
            center_x + self.screen_center_x,
            center_y + self.screen_center_y,
        );
        let rect = Rect::from_center(center, width, height);

        self.canvas.set_draw_color(*color);
        self.canvas.draw_rect(rect).unwrap();
    }

    fn draw_text(&mut self, text: &String, x: i32, y: i32, color: &Color) {
        // TODO: Learn lifetime specifiers so this loaded font can be cached.
        let font = self.ttf.load_font("ARCADECLASSIC.TTF", 64).unwrap();

        let texture_creator = self.canvas.texture_creator();
        let surface = font.render(&text).blended(*color).unwrap();

        let texture = texture_creator
            .create_texture_from_surface(surface)
            .unwrap();

        self.canvas.set_draw_color(*color);

        let TextureQuery { width, height, .. } = texture.query();

        let center = Point::new(
            x + self.screen_center_x - (width as i32 / 2),
            y + self.screen_center_y - (height as i32 / 2),
        );
        let rect = Rect::from((center.x, center.y, width, height));

        self.canvas.copy(&texture, None, rect).unwrap();
    }
}
