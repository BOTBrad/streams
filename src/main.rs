extern crate rand;

mod game;
mod pieces;

use std::thread::sleep;
use std::time::Duration;

use game::Game;

fn main() {
  let tick_delay = Duration::from_millis(200);

  let mut game = Game::new();

  loop {
    game
      .update()
      .draw();

    sleep(tick_delay);
  }
}
