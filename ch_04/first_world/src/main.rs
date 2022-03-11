fn main() {
  let msg = String::from("Hello, Rust!");
    let first = first_word(&msg);
    println!("{}", first);

    println!("Using literal strings `{}`", first_word_literal_string("using literal strings"));
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}

fn first_word_literal_string(s: &str) -> &str {
  let bytes = s.as_bytes();
  
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}
