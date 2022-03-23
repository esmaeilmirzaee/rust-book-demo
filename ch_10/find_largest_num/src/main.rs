struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

struct Pointy<X1, Y1> {
  x: X1,
  y: Y1,
}

impl<X1, Y1> Pointy<X1, Y1> {
  fn mixup<X2, Y2>(self, other: Pointy<X2, Y2>) -> Pointy<X1, Y2> {
    Pointy {
      x: self.x,
      y: other.y,
    }
  }
}

fn main() {
  let number_list = vec![1,2,3,4,5,10,1,20,2,3,12];
  let number = find_largest(&number_list);
  println!("{}", number);

  let char_list = vec!['a', 'b', 'A', 'c', 'z'];
  println!("The largest char is {}", find_largest(&char_list));

  let p = Point {x: 1, y: 2};
  println!("x is {}", &p.x());

  let p_float = Point{x: 1.1, y: 1.2};
  println!("p_float distance from (0.0, 0.0) is {}", p_float.distance_from_origin());
}

fn find_largest<T: PartialOrd + Copy>(v: &[T]) -> T {
  let mut max = v[0];
  for &item in v {
    if item > max {
      max = item;
    }
  }

  max
}
