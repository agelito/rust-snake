mod game_state;
mod game_window;
mod main_menu;
mod renderer;
mod resources;
mod snake_game;

use ::std::time::Duration;
use game_state::State;

extern crate sdl2;

use game_window::{GameWindow, GameWindowState};

fn main() {
    println!(
        "Welcome to {} application running on {}.",
        "snake",
        sdl2::get_platform()
    );

    resources::extract_resources(&[(
        "ARCADECLASSIC.TTF",
        include_bytes!("resources/ARCADECLASSIC.TTF"),
    )]);

    let mut game_window = GameWindow::create("snake", (800, 600));

    let mut state = State::Menu(main_menu::MainMenu::new());

    'running: loop {
        match game_window.event_loop() {
            GameWindowState::Closed => break 'running,
            _ => {}
        }

        state.handle_input(game_window.pressed_keys());

        state = match state.tick() {
            Some(s) => s,
            None => state,
        };

        match state {
            State::Quit => break 'running,
            _ => {}
        }

        game_window.clear();
        state.render(&mut game_window);
        game_window.redraw();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }

    println!("Exiting.");
}
