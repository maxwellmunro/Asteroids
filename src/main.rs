use crate::game::Game;

mod asteroid;
mod bullet;
mod constants;
mod game;
mod particle;
mod player;
mod polygon;
mod font;
mod high_score;
mod alien;

fn main() {
    let mut game = Game::new().unwrap_or_else(|e| panic!("{}", e));
    game.run();
}
