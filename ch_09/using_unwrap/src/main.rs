use std::fs::File;
use std::io::{self, Read};

fn main() {
  //let f = File::open("hello.txt").unwrap();
  let f = File::open("hello.txt").expect("Unable loading file");
}

fn shorter_read() -> Result<String, io::Error> {
  io::read_to_string("hello.txt")
}

fn read_username_from_file(file_name: &String) -> Result<String, io::Error> {
  let f = File::open(&file_name);

  let mut f = match f {
    Ok(file) => file,
    Err(error) => return Err(error),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}
