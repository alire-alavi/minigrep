use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let result = Config::build(&args)
        .unwrap_or_else(|err| {
            print!("Problem Parsing arguments: {err}");
            process::exit(1);
        });

    run(result);
}

fn run(config: Config) {
    let file_content = fs::read_to_string(config.file_path)
        .expect("Hmm there was a problem reading the file");

    println!("file contains: \n {}", file_content);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
   fn build(args: &[String]) -> Result<Config, &'static str> { 
    if args.len() < 3 {
        return Err("not enough arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();

    Ok(Config {query, file_path})
   }
}
