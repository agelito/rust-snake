use super::main_menu::MainMenu;
use super::renderer::Renderer;
use super::snake_game::SnakeGame;
use sdl2::keyboard::Keycode;

pub enum State {
    Menu(MainMenu),
    Play(SnakeGame),
    Quit,
}

impl State {
    pub fn handle_input(&mut self, keys: &Vec<Keycode>) {
        match self {
            State::Menu(main_menu) => main_menu.handle_input(keys),
            State::Play(snake_game) => snake_game.handle_input(keys),
            State::Quit => {}
        }
    }

    pub fn tick(&mut self) -> Option<State> {
        match self {
            State::Menu(menu) => menu.tick(),
            State::Play(snake_game) => snake_game.tick(),
            State::Quit => Some(State::Quit),
        }
    }

    pub fn render(&self, renderer: &mut dyn Renderer) {
        match self {
            State::Menu(main_menu) => {
                main_menu.render(renderer);
            }
            State::Play(snake_game) => {
                snake_game.render(renderer);
            }
            State::Quit => {}
        }
    }
}
