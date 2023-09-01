use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let result = Config::build(&args)
        .unwrap_or_else(|err| {
            eprintln!("Problem Parsing arguments: {err}");
            process::exit(1);
        });

    if let Err(e) = minigrep::run(result) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

