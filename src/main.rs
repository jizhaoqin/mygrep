use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = mygrep::Config::new(args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let _ = mygrep::run(config).unwrap_or_else(|err| {
        println!("Error found when opening file: {}", err);
        process::exit(1);
    });
}
