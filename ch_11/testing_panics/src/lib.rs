pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Value must be between 1 and 100, but got {}", value);
    }

    Guess {
      value,
    }
  }

  pub fn get(g: &Guess) -> i32 {
    g.value
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn greater_than_100() {
    Guess::new(500);
  }

  #[test]
  fn test_guessing() {
    let g = Guess::new(5);
    assert_eq!(5, Guess::get(&g));

  }
}
