use super::game_state::State;
use super::main_menu::MainMenu;
use crate::renderer::Renderer;

use rand::Rng;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn get_vector(&self) -> Point {
        match self {
            Direction::North => Point { x: 0, y: -1 },
            Direction::South => Point { x: 0, y: 1 },
            Direction::East => Point { x: 1, y: 0 },
            Direction::West => Point { x: -1, y: 0 },
        }
    }
}

pub struct Snake {
    parts: Vec<Point>,
    direction: Direction,
}

impl Snake {
    pub fn new(head: Point) -> Snake {
        Snake {
            parts: vec![head],
            direction: Direction::North,
        }
    }

    pub fn get_next_position(&self, tiles_x: i32, tiles_y: i32) -> Point {
        let direction = self.direction.get_vector();
        let head = self.parts.get(0).unwrap_or(&direction);

        Point {
            x: Self::wrap(head.x + direction.x, tiles_x),
            y: Self::wrap(head.y + direction.y, tiles_y),
        }
    }

    pub fn move_to(&mut self, next: Point, expand: bool) -> Option<Point> {
        match self.parts.iter().position(|p| *p == next) {
            Some(_index) => return None,
            None => {}
        }

        self.parts.insert(0, next);

        if !expand {
            self.parts.pop();
        }

        Some(next)
    }

    fn wrap(v: i32, max: i32) -> i32 {
        if v > max {
            0
        } else if v < 0 {
            max
        } else {
            v
        }
    }
}

pub struct SnakeGame {
    tiles_x: i32,
    tiles_y: i32,
    tile_size: i32,

    game_ticks: u32,
    ticks_per_movement: u32,

    snake: Snake,
    food: Vec<Point>,

    score: u32,
}

impl SnakeGame {
    pub fn new(tiles_x: i32, tiles_y: i32) -> SnakeGame {
        let mut rng = rand::thread_rng();
        let food: Vec<Point> = (0..4)
            .map(|_| Point {
                x: rng.gen_range(0, tiles_x),
                y: rng.gen_range(0, tiles_y),
            })
            .collect();

        let snake_head = Point {
            x: tiles_x / 2,
            y: tiles_y / 2,
        };

        SnakeGame {
            tiles_x,
            tiles_y,
            tile_size: 32,
            game_ticks: 0,
            ticks_per_movement: 30,
            snake: Snake::new(snake_head),
            food,
            score: 0,
        }
    }

    fn give_score(&mut self) {
        if self.ticks_per_movement > 5 {
            self.ticks_per_movement -= 1;
        }
        self.score += 1;
    }

    pub fn tick(&mut self) -> Option<State> {
        self.game_ticks = self.game_ticks + 1;
        if self.game_ticks >= self.ticks_per_movement {
            self.game_ticks = 0;

            let next_head = self.snake.get_next_position(self.tiles_x, self.tiles_y);

            // TODO: Currently this assumes there's maximum only one food on each tile.
            let eat = match self.food.iter().position(|f| *f == next_head) {
                Some(fi) => {
                    self.give_score();

                    self.food.swap_remove(fi);
                    self.generate_food();

                    true
                }
                None => false,
            };

            match self.snake.move_to(next_head, eat) {
                None => return Some(State::Menu(MainMenu::new())),
                _ => {}
            };
        }

        None
    }
    pub fn handle_input(&mut self, keys: &Vec<Keycode>) {
        for key in keys {
            match key {
                Keycode::Up => self.change_direction(Direction::North),
                Keycode::Down => self.change_direction(Direction::South),
                Keycode::Left => self.change_direction(Direction::West),
                Keycode::Right => self.change_direction(Direction::East),
                _ => {}
            }
        }
    }

    pub fn render(&self, renderer: &mut dyn Renderer) {
        self.render_world(renderer);
        self.render_food(renderer);
        self.render_snake(renderer);
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.snake.direction = direction
    }

    fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();
        let position = Point {
            x: rng.gen_range(0, self.tiles_x),
            y: rng.gen_range(0, self.tiles_y),
        };

        self.food.push(position);
    }

    fn render_world(&self, renderer: &mut dyn Renderer) {
        let top = -(self.tiles_y * self.tile_size + self.tile_size) / 2;

        renderer.draw_rect(
            0,
            0,
            (self.tiles_x * self.tile_size + self.tile_size) as u32,
            (self.tiles_y * self.tile_size + self.tile_size) as u32,
            &Color::WHITE,
        );

        renderer.draw_text(&format!("score {}", self.score), 0, top - 24, &Color::WHITE)
    }

    fn render_snake(&self, renderer: &mut dyn Renderer) {
        let tile_size = self.tile_size;
        for part in &self.snake.parts {
            let position = self.to_world(part);

            renderer.draw_rect(
                position.x,
                position.y,
                tile_size as u32,
                tile_size as u32,
                &Color::BLUE,
            )
        }
    }

    fn render_food(&self, renderer: &mut dyn Renderer) {
        for food in &self.food {
            let position = self.to_world(food);

            renderer.draw_rect(
                position.x,
                position.y,
                (self.tile_size / 2) as u32,
                (self.tile_size / 2) as u32,
                &Color::RED,
            )
        }
    }

    fn to_world(&self, point: &Point) -> Point {
        let half_size_x = self.tiles_x * self.tile_size / 2;
        let half_size_y = self.tiles_y * self.tile_size / 2;

        Point {
            x: -half_size_x + point.x * self.tile_size,
            y: -half_size_y + point.y * self.tile_size,
        }
    }
}
