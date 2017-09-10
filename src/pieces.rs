use rand::{Rng, StdRng};

#[derive(Debug)]
pub enum Piece {
  Left,
  Up,
  Down,
  Right,
}

pub struct PieceGen {
  rng: StdRng,
}

impl PieceGen {
  pub fn new() -> PieceGen {
    PieceGen {
      rng: StdRng::new().unwrap(),
    }
  }

  pub fn next(&mut self) -> Piece {
    match self.rng.gen_range::<u8>(0, 4) {
      0 => Piece::Left,
      1 => Piece::Up,
      2 => Piece::Down,
      3 => Piece::Right,
      _ => unreachable!(),
    }
  }
}
