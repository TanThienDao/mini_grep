use std::error::Error;
use std::{env, fs, process};
use minigrep::search;

fn main() {
    // parse the argument
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Searching for '{}' in file '{}'",
        config.query, config.file_path
    );
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{}", contents);
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}
struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        } else if args.len() > 3 {
            return Err("too many arguments");
        }

        let query = args.get(1).expect("no query string provided").to_string();
        let file_path = args.get(2).expect("no file path provided").to_string();
        Ok(Self { query, file_path })
    }
}
