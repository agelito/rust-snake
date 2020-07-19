use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

use crate::renderer::Renderer;

pub struct SdlGameWindow {
    // context: Sdl,
    // video: VideoSubsystem,
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

impl SdlGameWindow {
    pub fn create(title: &str, initial_size: (u32, u32)) -> SdlGameWindow {
        let context = sdl2::init().unwrap();
        let video = context.video().unwrap();

        let window = video
            .window(title, initial_size.0, initial_size.1)
            .position_centered()
            .resizable()
            .allow_highdpi()
            .build()
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        let event_pump = context.event_pump().unwrap();

        let center_x = (initial_size.0 / 2) as i32;
        let center_y = (initial_size.1 / 2) as i32;
        SdlGameWindow {
            canvas,
            event_pump,
            screen_center_x: center_x,
            screen_center_y: center_y,
            pressed_keys: Vec::new(),
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.canvas.window().size()
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

impl Renderer for SdlGameWindow {
    fn push_rect(&mut self, center_x: i32, center_y: i32, width: u32, height: u32, color: &Color) {
        let center = Point::new(
            center_x + self.screen_center_x,
            center_y + self.screen_center_y,
        );
        let rect = Rect::from_center(center, width, height);

        self.canvas.set_draw_color(*color);
        self.canvas.draw_rect(rect).unwrap();
    }
}
