use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::*;
use piston::window::{NoWindow, WindowSettings};

use pieces::{Piece, PieceGen};

pub struct Game {
  window: NoWindow,
  tick_count: u64,

  piece_gen: PieceGen,
  cur_piece: Piece,
}

impl Game {
  pub fn new() -> Game {
    Game {
      window: WindowSettings::new(
        "streams",
        [64, 64],
      ).build()
      .unwrap(),
      tick_count: 0,
      piece_gen: PieceGen::new(),
      cur_piece: Piece::Left,
    }
  }

  pub fn run(&mut self, fps: u64) {
    let mut events = Events::new(EventSettings::new()
      .max_fps(fps)
      .ups(fps)
    );

    while let Some(e) = events.next(&mut self.window) {
      if let Some(_) = e.render_args() {
        self.draw();
      }

      if let Some(_) = e.update_args() {
        self.update();
      }
    }
  }

  fn update(&mut self) -> &mut Self {
    self.tick_count += 1;
    self.cur_piece = self.piece_gen.next();

    self
  }

  pub fn draw(&self) {
    println!("{:?}\t{}", self.cur_piece, self.tick_count);
  }
}
