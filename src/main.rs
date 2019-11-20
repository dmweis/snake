use quicksilver::{
    geom::{Vector, Rectangle, Shape, Transform},
    graphics::{
        Background::{Col, Img},
        Color, Font, FontStyle
    },
    
    input::{Key, ButtonState},
    lifecycle::{Asset, run, State, Window, Settings},
    Result,
};
use instant::Instant;
use rand::{rngs::ThreadRng, Rng};
use std::collections::VecDeque;

const GRID_SIZE: i32 = 20;

trait ButtonPressed {
    fn was_pressed(&self) -> bool;
}

impl ButtonPressed for ButtonState {
    fn was_pressed(&self) -> bool {
        if let ButtonState::Pressed = self {
            true
        } else {
            false
        }
    }
}

struct Snake {
    position: VecDeque<Vector>,
    heading: Vector,
    last_move: Instant,
    alive: bool
}

fn modulo(a: i32, n: i32) -> i32 {
    (a % n + n) % n
}

impl Snake {
    fn new(position: Vector) -> Snake {
        let mut positions = VecDeque::new();
        positions.push_front(position);
        Snake {
            position: positions,
            heading: Vector::new(1, 0),
            last_move: Instant::now(),
            alive: true,
        }
    }

    fn try_move(&mut self, food: &Vector) -> bool {
        if !self.alive {
            return false;
        }
        if self.last_move.elapsed().as_millis() > 200 {
            let head = self.position.front();
            let mut new_head = if let Some(head) = head {
                self.heading + *head
            } else {
                Vector::new(0, 0)
            };
            new_head = Vector::new(modulo(new_head.x as i32, GRID_SIZE), modulo(new_head.y as i32, GRID_SIZE));
            self.position.push_front(new_head);
            self.last_move = Instant::now();

            if self.is_in_body(&new_head) {
                self.alive = false;
                return false;
            }

            if food.distance(new_head) < (0.5f32).powi(2) {
                return true;
            }
            self.position.pop_back();
        }
        false
    }

    fn is_in_body(& self, point: &Vector) -> bool {
        let point = point.clone();
        for element in self.position.iter().skip(1) {
            if element.distance(point) < (0.5f32).powi(2) {
                return true;
            }
        }
        false
    }
}

struct ColorPalette {
    background: Color,
    background_light: Color,
    snake_head: Color,
    snake_light: Color,
    food: Color,
    text: Color,
}

impl ColorPalette {
    fn new() -> ColorPalette {
        ColorPalette {
            background: Color::from_hex("#3E496B"),
            background_light: Color::from_hex("#646F91"),
            snake_head: Color::from_hex("#F25218"),
            snake_light: Color::from_hex("#C7A193"),
            food: Color::from_hex("#02A171"),
            text: Color::from_hex("#BCE6D9"),
        }
    }
}

struct Game {
    font: Asset<Font>,
    snake: Snake,
    food: Vector,
    rng: ThreadRng,
    score: i32,
    colors: ColorPalette
}

impl Game {
    fn restart(&mut self) {
        self.score = 0;
        self.snake = Snake::new(Vector::new(10, 10));
        self.food = Vector::new(self.rng.gen_range(0, GRID_SIZE), self.rng.gen_range(0, GRID_SIZE));
    }
}

impl State for Game {
    fn new() -> Result<Game> {
        let mut rng = rand::thread_rng();
        let font = Asset::new(Font::load("ShareTechMono-Regular.ttf"));
        Ok(Game {
            font,
            snake: Snake::new(Vector::new(10, 10)),
            food: Vector::new(rng.gen_range(0, GRID_SIZE), rng.gen_range(0, GRID_SIZE)),
            rng: rng,
            score: 0,
            colors: ColorPalette::new(),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Left].was_pressed() || window.keyboard()[Key::A].was_pressed(){
            self.snake.heading = Vector::new(-1, 0);
        }
        if window.keyboard()[Key::Right].was_pressed() || window.keyboard()[Key::D].was_pressed(){
            self.snake.heading = Vector::new(1, 0);
        }
        if window.keyboard()[Key::Up].was_pressed() || window.keyboard()[Key::W].was_pressed(){
            self.snake.heading = Vector::new(0, -1);
        }
        if window.keyboard()[Key::Down].was_pressed() || window.keyboard()[Key::S].was_pressed(){
            self.snake.heading = Vector::new(0, 1);
        }
        if window.keyboard()[Key::R].was_pressed() {
            self.restart();
        }
        let ate = self.snake.try_move(&self.food);
        if ate {
            self.score += 1;
            loop {
                self.food = Vector::new(self.rng.gen_range(0, GRID_SIZE), self.rng.gen_range(0, GRID_SIZE));
                if !self.snake.is_in_body(&self.food) {
                    break;
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(self.colors.background)?;
        let size = window.screen_size();

        let height = size.x / GRID_SIZE as f32;
        let width = size.y / GRID_SIZE as f32;
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                let square = Rectangle::new((x as f32 * width + 2.5, y as f32 * height + 2.5), (width - 5.0, height - 5.0));
                window.draw(&square, Col(self.colors.background_light));
            }
        }

        let food = Rectangle::new((self.food.x * width + 2.5, self.food.y * height + 2.5), (width - 5.0, height - 5.0));
        window.draw(&food, Col(self.colors.food));

        for element in self.snake.position.iter().skip(1) {
            let snake_element = Rectangle::new((element.x * width + 2.5, element.y * height + 2.5), (width - 5.0, height - 5.0));
            window.draw(&snake_element, Col(self.colors.snake_light));
        }
        let snake_head = Rectangle::new((self.snake.position[0].x * width + 2.5, self.snake.position[0].y * height + 2.5), (width - 5.0, height - 5.0));
        window.draw(&snake_head, Col(self.colors.snake_head));
        let lost = !self.snake.alive;
        let score = self.score;
        let text_color = self.colors.text;
        let background_color = self.colors.background;
        self.font.execute(move |font| {
            if lost {
                let style = FontStyle::new(60.0, text_color);
                let text = format!("YOU LOST!\nYour score was {}", score);
                let image = font.render(&text, &style).unwrap();
                let text_point = (size.x / 2.0, size.y / 2.0);
                // draw background
                let background_area = image.area();
                let text_background = Rectangle::new_sized(background_area.size() * 1.2).with_center(text_point);
                window.draw(&text_background, Col(background_color));
                window.draw_ex(&image.area().with_center(text_point), Img(&image), Transform::IDENTITY, 10);
            }
            Ok(())
        })?;

        Ok(())
    }
}

fn main() {
    run::<Game>("Snake", Vector::new(1000, 1000), Settings::default());
}
