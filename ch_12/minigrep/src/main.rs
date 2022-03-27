use std::env;
use std::error::Error;
use std::fs;
use std::ops::Add;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}.", err);
        process::exit(1);
    });


    println!("{} {}", config.query, config.filename);

    if let Err(e) = run(&mut config) {
        println!("Application error {}.", e);
        process::exit(1);
    };
}

fn run(config: &mut Config) -> Result<(), Box<dyn Error>> {
    if !config.filename.contains(".txt") {
        println!("Adding extension type!");
        config.filename = config.filename.clone().add(&String::from(".txt"));
    }
    let content = fs::read_to_string(&config.filename)?;

    println!("With text:\n{}", content);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Please provide `query` and the `filename`");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}
