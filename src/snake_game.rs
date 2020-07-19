use crate::renderer::Renderer;

use rand::Rng;
use sdl2::pixels::Color;

pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Snake {
    parts: Vec<Point>,
    direction: Direction,
    food_eaten: u32,
}

impl Snake {
    pub fn new(head: Point) -> Snake {
        Snake {
            parts: vec![head],
            direction: Direction::North,
            food_eaten: 0,
        }
    }

    pub fn eat(&mut self) {
        self.food_eaten = self.food_eaten + 1;
    }

    pub fn movement(&mut self, tiles_x: i32, tiles_y: i32) -> Point {
        let head = self.parts.get(0).unwrap();
        let head = match self.direction {
            Direction::North => Point {
                x: head.x,
                y: Self::wrap(head.y - 1, tiles_y),
            },
            Direction::South => Point {
                x: head.x,
                y: Self::wrap(head.y + 1, tiles_y),
            },
            Direction::East => Point {
                x: Self::wrap(head.x + 1, tiles_x),
                y: head.y,
            },
            Direction::West => Point {
                x: Self::wrap(head.x - 1, tiles_x),
                y: head.y,
            },
        };

        match self
            .parts
            .iter()
            .position(|p| p.x == head.x && p.y == head.y)
        {
            Some(index) => self.parts.truncate(index + 1),
            None => {}
        }

        self.parts.insert(0, head);

        if self.food_eaten == 0 {
            self.parts.pop();
        } else {
            self.food_eaten = self.food_eaten - 1;
        }

        head
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
    tiles_x: u32,
    tiles_y: u32,
    tile_size: u32,

    game_ticks: u32,
    ticks_per_movement: u32,

    snake: Snake,
    food: Vec<Point>,
}

impl SnakeGame {
    pub fn new(tiles_x: u32, tiles_y: u32) -> SnakeGame {
        let mut rng = rand::thread_rng();
        let food: Vec<Point> = (0..4)
            .map(|_| Point {
                x: rng.gen_range(0, tiles_x as i32),
                y: rng.gen_range(0, tiles_y as i32),
            })
            .collect();

        SnakeGame {
            tiles_x,
            tiles_y,
            tile_size: 32,
            game_ticks: 0,
            ticks_per_movement: 30,
            snake: Snake::new(Point {
                x: (tiles_x / 2) as i32,
                y: (tiles_y / 2) as i32,
            }),
            food,
        }
    }

    pub fn tick(&mut self) {
        self.game_ticks = self.game_ticks + 1;
        if self.game_ticks >= self.ticks_per_movement {
            self.game_ticks = 0;

            let head = self
                .snake
                .movement(self.tiles_x as i32, self.tiles_y as i32);

            // TODO: Currently this assumes there's maximum only one food on each tile.
            match self
                .food
                .iter()
                .position(|f| f.x == head.x && f.y == head.y)
            {
                Some(fi) => {
                    self.snake.eat();
                    self.food.swap_remove(fi);
                    self.food.push(self.generate_food())
                }
                None => {}
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

    fn generate_food(&self) -> Point {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0, self.tiles_x as i32),
            y: rng.gen_range(0, self.tiles_y as i32),
        }
    }

    fn render_world(&self, renderer: &mut dyn Renderer) {
        renderer.push_rect(
            0,
            0,
            self.tiles_x * self.tile_size + self.tile_size,
            self.tiles_y * self.tile_size + self.tile_size,
            &Color::WHITE,
        );
    }

    fn render_snake(&self, renderer: &mut dyn Renderer) {
        let tile_size = self.tile_size;
        for part in &self.snake.parts {
            let position = self.to_world(part);

            renderer.push_rect(position.x, position.y, tile_size, tile_size, &Color::BLUE)
        }
    }

    fn render_food(&self, renderer: &mut dyn Renderer) {
        for food in &self.food {
            let position = self.to_world(food);

            renderer.push_rect(
                position.x,
                position.y,
                self.tile_size / 2,
                self.tile_size / 2,
                &Color::RED,
            )
        }
    }

    fn to_world(&self, point: &Point) -> Point {
        let half_size_x = self.tiles_x * self.tile_size / 2;
        let half_size_y = self.tiles_y * self.tile_size / 2;

        Point {
            x: -(half_size_x as i32) + point.x * self.tile_size as i32,
            y: -(half_size_y as i32) + point.y * self.tile_size as i32,
        }
    }
}
