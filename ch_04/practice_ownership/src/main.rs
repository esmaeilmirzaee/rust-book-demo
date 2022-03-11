fn main() {
    let x = 5;
    take_ownership(x);
    println!("{}", x);

    let s = String::from("Hello");
    another_ownership(s);
    // println!("{}", s); // value borrowed here after move

    // sending and receiving back ownership of a variable
    let s1 = String::from("Hello, Rust");
    println!("{}", s1);
    let s2 = take_and_receive_ownership(s1);
    println!("{}", s2);

    // reference to mitigate a lot of ceremony
    let (count, msg) = takes_a_reference(&s2);
    println!("S2 = {}", s2);
    println!("Message {} costs ${}", msg, count);
}

fn take_ownership(x: u32) {
  println!("{}", x);
}

fn another_ownership(s: String) {
  println!("{}", s);
}

fn take_and_receive_ownership(s: String) -> String {
  println!("{}", s);

  s
}

fn takes_a_reference(s: &String) -> (usize, String) {
  println!("{}", s);
  let mut msg = String::from("Lorem ipsum");

  let count = alters_using_reference(&mut msg);
  
  (count, msg)
}

// reference is a immutable by default
fn alters_using_reference(s: &mut String) -> usize {
  s.push_str("!");

  s.len()
}
