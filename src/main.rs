use quicksilver::{
    geom::Vector,
    lifecycle::{run, State, Window, Settings},
    Result,
};

struct Game;

impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }
}

fn main() {
    run::<Game>("Snake", Vector::new(800, 600), Settings::default());
}
