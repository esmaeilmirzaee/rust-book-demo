struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
    println!("Hello, world!");
    let r = Rectangle {
      width: 20,
      height: 10,
    };

    println!("The area of a rectangle {}x{} is {}.", r.width, r.height, rect_area(&r));
}

fn rect_area(r: &Rectangle) -> u64 {
  (r.width * r.height).into()
}
