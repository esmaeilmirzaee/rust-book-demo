enum SpreadSheet {
  Int(i32),
  Float(f64),
  Text(String),
}

fn main() {
  let row = vec![
    SpreadSheet::Int(3),
    SpreadSheet::Float(3.1415),
    SpreadSheet::Text(String::from("Hello Rust")),
  ];

  println!("{:?}", row);
}
