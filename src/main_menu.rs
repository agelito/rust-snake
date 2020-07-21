use super::game_state::State;
use super::renderer::Renderer;
use super::snake_game::SnakeGame;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub struct MainMenu {
    selected: usize,
    buttons: Vec<String>,

    color: Color,
    selected_color: Color,

    play: bool,
    quit: bool,
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            selected: 0,
            buttons: [String::from("Play"), String::from("Quit")].to_vec(),
            color: Color::WHITE,
            selected_color: Color::BLUE,
            play: false,
            quit: false,
        }
    }

    pub fn handle_input(&mut self, keys: &Vec<Keycode>) {
        for key in keys {
            match key {
                Keycode::Up | Keycode::W => self.move_up(),
                Keycode::Down | Keycode::S => self.move_down(),
                Keycode::Return => {
                    self.play = self.selected == 0;
                    self.quit = self.selected == 1;
                }
                _ => {}
            }
        }
    }

    pub fn tick(&mut self) -> Option<State> {
        if self.play {
            Some(State::Play(SnakeGame::new(16, 16)))
        } else if self.quit {
            Some(State::Quit)
        } else {
            None
        }
    }

    fn move_down(&mut self) {
        let mut selected = self.selected + 1;
        if selected >= self.buttons.len() {
            selected = 0;
        }

        self.selected = selected;
    }

    fn move_up(&mut self) {
        if self.selected == 0 {
            self.selected = self.buttons.len() - 1
        } else {
            self.selected -= 1;
        }
    }

    pub fn render(&self, renderer: &mut dyn Renderer) {
        let button_space = 100i32;
        let offset = (button_space * self.buttons.len() as i32) / 2;

        for (index, button) in self.buttons.iter().enumerate() {
            let color = if index == self.selected {
                self.selected_color
            } else {
                self.color
            };
            renderer.draw_text(&button, 0, (index as i32 * button_space) - offset, &color);
        }
    }
}
