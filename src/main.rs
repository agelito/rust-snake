mod renderer;
mod sdl_game_window;
mod snake_game;

use ::std::time::Duration;
use sdl2::keyboard::Keycode;
use snake_game::{Direction, SnakeGame};

extern crate sdl2;

use sdl_game_window::{GameWindowState, SdlGameWindow};

fn main() {
    println!(
        "Welcome to {} application running on {}.",
        "game"",
        sdl2::get_platform()
    );

    let mut game_window = SdlGameWindow::create("raytracer2", (800, 600));

    let mut snake_game = SnakeGame::new(16, 16);

    let (width, height) = game_window.size();
    println!("Created window of size: {}, {}", width, height);

    'running: loop {
        match game_window.event_loop() {
            GameWindowState::Closed => break 'running,
            _ => {}
        }

        let keys = game_window.pressed_keys();
        for key in keys {
            match key {
                Keycode::Up => snake_game.change_direction(Direction::North),
                Keycode::Down => snake_game.change_direction(Direction::South),
                Keycode::Left => snake_game.change_direction(Direction::West),
                Keycode::Right => snake_game.change_direction(Direction::East),
                _ => {}
            }
        }

        snake_game.tick();

        game_window.clear();
        snake_game.render(&mut game_window);
        game_window.redraw();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000_u32 / 60));
    }

    println!("Exiting.");
}
