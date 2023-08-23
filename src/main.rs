use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::time::Duration;


const TILE_SIZE: u32 = 20;
const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: Vec<(i32, i32)>,
    direction: Direction,
}

impl Snake {
    fn move_forward(&mut self) {
        let (dx, dy) = match self.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let (head_x, head_y) = self.body[0];
        self.body.insert(0, (head_x + dx, head_y + dy));
        self.body.pop();
    }

    fn grow(&mut self) {
        let (dx, dy) = match self.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let (head_x, head_y) = self.body[0];
        self.body.insert(0, (head_x + dx, head_y + dy));
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Snake Game", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut snake = Snake {
        body: vec![(10, 10), (10, 11), (10, 12)],
        direction: Direction::Up,
    };
    let mut food = (15, 15);

    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } |
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    snake.direction = Direction::Up;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    snake.direction = Direction::Down;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    snake.direction = Direction::Left;
                },
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    snake.direction = Direction::Right;
                },
                _ => {}
            }
        }

        snake.move_forward();

        // Collision with wall
        let (head_x, head_y) = snake.body[0];
        if head_x < 0 || head_y < 0 || head_x as u32 >= WIDTH / TILE_SIZE || head_y as u32 >= HEIGHT / TILE_SIZE {
            break 'game_loop;
        }

        // Collision with itself
        for &(x, y) in &snake.body[1..] {
            if (x, y) == (head_x, head_y) {
                break 'game_loop;
            }
        }

        // Collision with food
        if (head_x, head_y) == food {
            snake.grow();
            // For simplicity, place food at a random position (this could lead to food being inside the snake, in a real game you'd handle this differently)
            food = ((rand::random::<u32>() % (WIDTH / TILE_SIZE)) as i32, (rand::random::<u32>() % (HEIGHT / TILE_SIZE)) as i32);
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Drawing snake
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        for &(x, y) in &snake.body {
            canvas.fill_rect(Rect::new(x * TILE_SIZE as i32, y * TILE_SIZE as i32, TILE_SIZE, TILE_SIZE)).unwrap();
        }

        // Drawing food
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(Rect::new(food.0 * TILE_SIZE as i32, food.1 * TILE_SIZE as i32, TILE_SIZE, TILE_SIZE)).unwrap();

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }
}
