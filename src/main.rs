use std::process;
use std::env;

use clip::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    if let Err(error) = clip::run(config) {
        eprintln!("Error occured while running: {}", error);

        process::exit(1);
    }
}
