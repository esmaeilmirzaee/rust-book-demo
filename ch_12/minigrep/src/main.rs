// I'm wondering what I misunderstood about importing when I add the following line
// I see unused_warning, and leaving it out I see use of undeclared crate or module `env`
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}.", err);
        process::exit(1);
    });


    println!("{} {}", config.query, config.filename);

    if let Err(e) = minigrep::run(&mut config) {
        println!("Application error {}.", e);
        process::exit(1);
    };
}
