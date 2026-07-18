use minigrep::{search, search_case_insensitive};
use std::error::Error;
use std::{env, fs, process};

fn main() {
    // parse the argument
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Change: Now `run` returns the results, so we capture them in `search_results`
    let search_results = match run(config) {
        Ok(results) => results, // If Ok, extract the Vec<String>
        Err(e) => { // If Err, print the error and exit
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    };

    // Display the results received from the run function
    println!("\n--- Search Results ---");
    if search_results.is_empty() {
        println!("No matches found.");
    } else {
        for line in search_results {
            println!("{}", line);
        }
    }
    println!("----------------------");
}

fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    println!(
        "Searching for '{}' in file '{}'",
        config.query, config.file_path
    );
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        println!("Ignoring case");
        search_case_insensitive(&config.query, &contents)
    } else {
        println!("Not ignoring case");
        search(&config.query, &contents)
    };

    // Change: Return the actual results
    Ok(results)
}

struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args.get(1).expect("no query string provided").to_string();
        let file_path = args.get(2).expect("no file path provided").to_string();
        //let ignore_case = args.get(3).is_some();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        println!("ignore_case: {}", ignore_case);
        Ok(Self { query, file_path, ignore_case })
    }
}