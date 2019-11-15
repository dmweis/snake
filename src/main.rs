use quicksilver::{
    geom::{Vector, Rectangle},
    graphics::{
        Background::Col,
        Color
    },
    input::Key,
    lifecycle::{run, State, Window, Settings},
    Result,
};
use instant::Instant;
use rand::{rngs::ThreadRng, Rng};

const GRID_SIZE: i32 = 20;

struct Snake {
    position: Vec<Vector>,
    heading: Vector,
    last_move: Instant,
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
        }
    }

    fn try_move(&mut self, food: &Vector) -> bool {
        if self.last_move.elapsed().as_millis() > 200 {
            let head = self.position[0];
            let mut new_head = head + self.heading;
            new_head = Vector::new(modulo(new_head.x as i32, GRID_SIZE), modulo(new_head.y as i32, GRID_SIZE));
            self.position.insert(0, new_head);
            self.last_move = Instant::now();

            if food.distance(new_head) < (0.5f32).powi(2) {
                return true;
            }
            self.position.pop();
        }
        false
    }
}

struct Game {
    snake: Snake,
    food: Vector,
    rng: ThreadRng,
    score: i32,
}

impl State for Game {
    fn new() -> Result<Game> {
        let mut rng = rand::thread_rng();
        Ok(Game {
            snake: Snake::new(Vector::new(10, 10)),
            food: Vector::new(rng.gen_range(0, GRID_SIZE+1), rng.gen_range(0, GRID_SIZE+1)),
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
            self.food = Vector::new(self.rng.gen_range(0, GRID_SIZE), self.rng.gen_range(0, GRID_SIZE));
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

        Ok(())
    }
}

fn main() {
    run::<Game>("Snake", Vector::new(800, 800), Settings::default());
}
