use rust_grep::run;
use rust_grep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Failed to parse input: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("Application err: {}", err);
    }
}
