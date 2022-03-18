fn main() {
  let mut v = vec![1, 2, 3];
  v.push(10);
  
  let third: &i32 = &v[2];
  println!("The third element is {}.", third);

  match v.get(2) {
    Some(t) => println!("The third element is {}.", t),
    None => println!("There is no third element."),
  }

  
  println!("{:?}, [0]:[{}]", v, v[0]);

  // let's see what would Rust do when it doesn't have the value
  // causes panic because accessing an index that inexist
  //let does_not_exist = &v[100];
  //println!("{:?}", does_not_exist);
  //let does_not_exist = v.get(100);
  //println!("{:?}", does_not_exist);
  
  
  for item in &v {
    println!("{}", item);
  }

  for item in &mut v {
    *item += 50;
    println!("{}", item);
  }

  println!("{:?}", v);
}
