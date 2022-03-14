struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u64 {
    (self.width * self.height).into()
  }

  fn width(&self) -> bool {
    self.width > 0
  }
}

fn main() {
    println!("Hello, world!");

    let r = Rectangle {
      width: 15,
      height: 20,
    };

    println!("The area of a rectangle {}x{} equals {}.", r.width, r.height, r.area());
    println!("The width is greater than 0: {}.", r.width());
}
