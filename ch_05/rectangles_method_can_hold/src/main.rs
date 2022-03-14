struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, r: &Rectangle) -> bool {
    self.width < r.width && self.height < r.height
  }

  // associated function | constructor
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

fn main() {
  let r1 = Rectangle {
    width: 20,
    height: 15,
  };

  let r2 = Rectangle {
    width: 10,
    height: 20,
  };

  let r3 = Rectangle {
    width: 22,
    height: 16,
  };

  println!("r1 can hold r2: {}", r1.can_hold(&r2));
  println!("r1 can hold r3: {}", r1.can_hold(&r3));
  println!("r3 can hold r1: {}", r3.can_hold(&r1));
  
  // associated function
  let r4 = Rectangle::square(4);
  println!("A square rectangle: {}x{}={}", r4.width, r4.height, r4.area());
}
