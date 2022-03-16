enum Coin {
  Penny,
  Nikel,
  Dime,
  Quarter,
}

fn values_in_cents(c: Coin) -> u8 {
  match c {
    Coin::Penny => 1,
    Coin::Nikel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

fn main() {
    println!("Hello, world!");
}
