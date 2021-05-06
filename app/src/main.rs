use search_query::run;
use search_query::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse input: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application err: {}", err);
    }
}
