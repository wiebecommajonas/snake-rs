use rand::{thread_rng, Rng};
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::shape::Rectangle;
use speedy2d::window::{
    KeyScancode, VirtualKeyCode, WindowHandler, WindowHelper, WindowStartupInfo,
};
use speedy2d::{Graphics2D, Window};
use std::time;

const CELL_SIZE: u32 = 10;
const HEIGHT: u32 = 480;
const WIDTH: u32 = 640;

struct Handler {
    snake: Snake,
    start_time: time::Instant,
    food: Food,
}

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        match *self {
            Direction::UP => match *other {
                Direction::UP => true,
                _ => false,
            },
            Direction::RIGHT => match *other {
                Direction::RIGHT => true,
                _ => false,
            },
            Direction::DOWN => match *other {
                Direction::DOWN => true,
                _ => false,
            },
            Direction::LEFT => match *other {
                Direction::LEFT => true,
                _ => false,
            },
        }
    }
}

struct Snake {
    head: Vector2<f32>,
    direction: Direction,
    path: Vec<Vector2<f32>>,
}

struct Food {
    position: Vector2<f32>,
    color: Color,
}

impl WindowHandler for Handler {
    fn on_start(&mut self, _helper: &mut WindowHelper, _info: WindowStartupInfo) {
        self.snake = Snake::init();
    }
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        if self.start_time.elapsed().as_millis() >= 100 {
            graphics.clear_screen(Color::WHITE);

            if self.snake.head == self.food.position {
                self.snake.grow();
                self.food = Food::new();
            }
            self.snake.r#move();
            graphics.draw_food(&mut self.food);
            graphics.draw_snake(&mut self.snake);

            self.start_time = time::Instant::now();
        }
        helper.request_redraw();
    }
    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        let dir = &self.snake.direction;
        match virtual_key_code {
            Some(VirtualKeyCode::Up) => {
                if *dir != Direction::DOWN {
                    self.snake.direction = Direction::UP
                }
            }
            Some(VirtualKeyCode::Right) => {
                if *dir != Direction::LEFT {
                    self.snake.direction = Direction::RIGHT
                }
            }
            Some(VirtualKeyCode::Down) => {
                if *dir != Direction::UP {
                    self.snake.direction = Direction::DOWN
                }
            }
            Some(VirtualKeyCode::Left) => {
                if *dir != Direction::RIGHT {
                    self.snake.direction = Direction::LEFT
                }
            }
            _ => (),
        };
    }
}

trait CanDrawSnake {
    fn draw_snake(&mut self, snake: &mut Snake);
    fn draw_food(&mut self, food: &mut Food);
}

impl CanDrawSnake for Graphics2D {
    fn draw_snake(&mut self, snake: &mut Snake) {
        for p in snake.path.iter() {
            let cell = Rectangle::new(
                Vector2::new(p.x, p.y),
                Vector2::new(p.x + CELL_SIZE as f32, p.y + CELL_SIZE as f32),
            );
            self.draw_rectangle(cell, Color::BLACK);
        }
    }
    fn draw_food(&mut self, food: &mut Food) {
        let x = food.position.x;
        let y = food.position.y;
        let cell = Rectangle::new(
            Vector2::new(x, y),
            Vector2::new(x + CELL_SIZE as f32, y + CELL_SIZE as f32),
        );
        self.draw_rectangle(cell, food.color);
    }
}

impl Snake {
    fn init() -> Snake {
        let h = Vector2::new(0.0, 0.0);
        Snake {
            head: h,
            direction: Direction::RIGHT,
            path: vec![h],
        }
    }
    fn r#move(&mut self) {
        let front = self.head;
        let new = match self.direction {
            Direction::UP => Vector2::new(front.x, front.y - CELL_SIZE as f32),
            Direction::RIGHT => Vector2::new(front.x + CELL_SIZE as f32, front.y),
            Direction::DOWN => Vector2::new(front.x, front.y + CELL_SIZE as f32),
            Direction::LEFT => Vector2::new(front.x - CELL_SIZE as f32, front.y),
        };
        self.path.remove(self.path.len() - 1);
        self.head = new;
        self.path.insert(0, new);
    }
    fn grow(&mut self) {
        let front = self.head;
        let new = match self.direction {
            Direction::UP => Vector2::new(front.x, front.y - CELL_SIZE as f32),
            Direction::RIGHT => Vector2::new(front.x + CELL_SIZE as f32, front.y),
            Direction::DOWN => Vector2::new(front.x, front.y + CELL_SIZE as f32),
            Direction::LEFT => Vector2::new(front.x - CELL_SIZE as f32, front.y),
        };
        self.head = new;
        self.path.insert(0, new);
    }
}

impl Food {
    fn new() -> Food {
        let mut x: u32 = thread_rng().gen_range(0..=WIDTH);
        if x % CELL_SIZE > CELL_SIZE / 2 {
            x += CELL_SIZE - x % CELL_SIZE;
        } else {
            x -= CELL_SIZE + x % CELL_SIZE;
        }
        let mut y: u32 = thread_rng().gen_range(0..=HEIGHT);
        if y % CELL_SIZE > CELL_SIZE / 2 {
            y += CELL_SIZE - y % CELL_SIZE;
        } else {
            y -= CELL_SIZE + y % CELL_SIZE;
        }
        Food {
            position: Vector2::new(x as f32, y as f32),
            color: Color::RED,
        }
    }
}

fn main() {
    let window = Window::new_centered("Snake", (WIDTH, HEIGHT)).unwrap();

    window.run_loop(Handler {
        snake: Snake::init(),
        food: Food::new(),
        start_time: time::Instant::now(),
    });
}
