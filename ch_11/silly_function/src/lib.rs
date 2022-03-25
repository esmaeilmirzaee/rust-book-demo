pub mod silly {
  pub fn prints_and_returns_10(a: i32) -> u32 {
    println!("{}", a);
    10
  }
}


#[cfg(test)]
mod tests {
  use crate::silly::prints_and_returns_10;


  #[test]
  fn tests_prints_and_returns_10() {
    let v = prints_and_returns_10(10);
    assert_eq!(10, v);
  }

  #[test]
  fn tests_prints_and_returns_10_ne() {
    let v = prints_and_returns_10(4);
    assert_ne!(15, v);
  }
}
