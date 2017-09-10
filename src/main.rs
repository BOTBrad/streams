use std::thread::sleep;
use std::time::Duration;

mod game;

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
