extern crate rand;
extern crate piston;

mod game;
mod pieces;

use game::Game;

fn main() {
  Game::new().run(10);
}
