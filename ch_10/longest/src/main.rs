fn longest<'a>(str1: &'a String, str2: &'a String) -> &'a String {
  if str1.len() > str2.len() {
    str1
  } else {
    str2
  } 
}

fn main() {
    println!("Hello, world!");
    let hello = String::from("Hello");
    let string2 = "Hello".to_string();
    let l = longest(&string2, &hello);
    println!("{}", l);
}
