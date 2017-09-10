pub struct Game {
  tick_count : i64,
}

impl Game {
  pub fn new() -> Game {
    Game {
      tick_count: 0,
    }
  }

  pub fn update(&mut self) -> &mut Self {
    self.tick_count += 1;
    self
  }

  pub fn draw(&self) {
    println!("{}", self.tick_count);
  }
}
