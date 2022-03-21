use rand::Rng;
use std::io;

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) ->Guess{
        if value < 0 || value > 100 {
            panic!("Guess value must be between 1 and 100, but got {}", value);
        }

        Guess {
            value,
        }
    }
}

fn main() {
    println!("Hello, to guessing game!");
    let secret_number = rand::thread_rng().rng(1..100);
    loop {
        let mut guess = String::new();

        let guess =  io::Stdin::read_line(&mut guess);
        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess.new(num),
            Err(e) => {
                println!("Something went wrong.");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Cmp::
        }
    }
}
