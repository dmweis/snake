use quicksilver::{
    geom::{Vector, Rectangle, Shape, Transform},
    graphics::{
        Background::{Col, Img},
        Color, Font, FontStyle
    },
    
    input::Key,
    lifecycle::{Asset, run, State, Window, Settings},
    Result,
};
use instant::Instant;
use rand::{rngs::ThreadRng, Rng};

const GRID_SIZE: i32 = 20;

struct Snake {
    position: Vec<Vector>,
    heading: Vector,
    last_move: Instant,
    alive: bool
}

fn modulo(a: i32, n: i32) -> i32 {
    (a % n + n) % n
}

impl Snake {
    fn new(position: Vector) -> Snake {
        Snake {
            position: vec![position],
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
            let head = self.position[0];
            let mut new_head = head + self.heading;
            new_head = Vector::new(modulo(new_head.x as i32, GRID_SIZE), modulo(new_head.y as i32, GRID_SIZE));
            self.position.insert(0, new_head);
            self.last_move = Instant::now();

            if self.is_in_body(&new_head) {
                self.alive = false;
                return false;
            }

            if food.distance(new_head) < (0.5f32).powi(2) {
                return true;
            }
            self.position.pop();
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

struct Game {
    font: Asset<Font>,
    snake: Snake,
    food: Vector,
    rng: ThreadRng,
    score: i32,
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
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Left].is_down() || window.keyboard()[Key::A].is_down(){
            self.snake.heading = Vector::new(-1, 0);
        }
        if window.keyboard()[Key::Right].is_down() || window.keyboard()[Key::D].is_down(){
            self.snake.heading = Vector::new(1, 0);
        }
        if window.keyboard()[Key::Up].is_down() || window.keyboard()[Key::W].is_down(){
            self.snake.heading = Vector::new(0, -1);
        }
        if window.keyboard()[Key::Down].is_down() || window.keyboard()[Key::S].is_down(){
            self.snake.heading = Vector::new(0, 1);
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
        window.clear(Color::BLACK)?;
        let size = window.screen_size();

        let height = size.x / GRID_SIZE as f32;
        let width = size.y / GRID_SIZE as f32;
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                let square = Rectangle::new((x as f32 * width + 2.5, y as f32 * height + 2.5), (width - 5.0, height - 5.0));
                window.draw(&square, Col(Color::BLUE));
            }
        }

        let food = Rectangle::new((self.food.x * width + 2.5, self.food.y * height + 2.5), (width - 5.0, height - 5.0));
        window.draw(&food, Col(Color::GREEN));

        for element in self.snake.position.iter().skip(1) {
            let snake_head = Rectangle::new((element.x * width + 2.5, element.y * height + 2.5), (width - 5.0, height - 5.0));
            window.draw(&snake_head, Col(Color::ORANGE));
        }
        let snake_head = Rectangle::new((self.snake.position[0].x * width + 2.5, self.snake.position[0].y * height + 2.5), (width - 5.0, height - 5.0));
        window.draw(&snake_head, Col(Color::RED));
        let lost = !self.snake.alive;
        self.font.execute(move |font| {
            if lost {
                let style = FontStyle::new(60.0, Color::RED);
                let text = "YOU LOST!";
                let image = font.render(&text, &style).unwrap();
                let text_point = (size.x / 2.0, size.y / 2.0);
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
