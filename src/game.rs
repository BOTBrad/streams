use pieces::{Piece, PieceGen};

pub struct Game {
  tick_count: i64,

  piece_gen: PieceGen,
  cur_piece: Piece,
}

impl Game {
  pub fn new() -> Game {
    Game {
      tick_count: 0,
      piece_gen: PieceGen::new(),
      cur_piece: Piece::Left,
    }
  }

  pub fn update(&mut self) -> &mut Self {
    self.tick_count += 1;
    self.cur_piece = self.piece_gen.next();

    self
  }

  pub fn draw(&self) {
    println!("{:?}\t{}", self.cur_piece, self.tick_count);
  }
}
