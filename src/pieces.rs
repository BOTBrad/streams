use rand::{Rng, StdRng};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
  Left,
  Up,
  Down,
  Right,
}

pub struct PieceGen {
  rng: StdRng,
  prev_piece: Piece,
  prev_foot: Piece,
}

impl PieceGen {
  pub fn new() -> PieceGen {
    PieceGen {
      rng: StdRng::new().unwrap(),
      prev_piece: Piece::Left,
      prev_foot: Piece::Left,
    }
  }

  pub fn next(&mut self) -> Piece {
    let pieces = self.piece_vec();

    // TODO: come up with a way to make this look good
    let next_piece = pieces[self
      .rng
      .gen_range::<usize>(0, pieces.len())
    ];

    self.prev_piece = next_piece;
    self.swap_feet();

    next_piece
  }

  fn swap_feet(&mut self) {
    self.prev_foot = match self.prev_foot {
      Piece::Left => Piece::Right,
      Piece::Right => Piece::Left,
      _ => unreachable!(),
    }
  }

  fn piece_vec(&self) -> Vec<Piece> {
    let all_pieces = vec![
      Piece::Left,
      Piece::Up,
      Piece::Down,
      Piece::Right,
    ];

    let mut pieces = Vec::<Piece>::new();
    for p in all_pieces.iter() {
      if *p != self.prev_piece
        && *p != self.prev_foot {
        pieces.push(*p);
      }
    }

    pieces
  }
}
