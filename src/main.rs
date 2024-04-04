use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let target = &args[1];
    // let file_path = &args[2];

    let config = Config::new(args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path).expect("problem found when opening file");
    println!("{}", contents);
}

struct Config {
    target: String,
    file_path: String,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let config = Config {
            target: args[1].clone(),
            file_path: args[2].clone(),
        };
        Ok(config)
    }
}
