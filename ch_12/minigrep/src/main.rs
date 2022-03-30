// I'm wondering what I misunderstood about importing when I add the following line
// I see unused_warning, and leaving it out I see use of undeclared crate or module `env`
use std::env;
use std::process;

fn main() {
    let mut config = minigrep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}.", err);
        process::exit(1);
    });


    if let Err(e) = minigrep::run(&mut config) {
        eprintln!("Application error {}.", e);
        process::exit(1);
    };
}
