use minigrep::{search, search_case_insensitive};
use std::error::Error;
use std::{env, fs, process};

fn main() {
    //let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Read the file contents here in main, so it lives long enough
    let contents = fs::read_to_string(&config.file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", config.file_path, err);
        process::exit(1);
    });

    // Pass a reference to contents to the run function
    let search_results = match run(config, &contents) {
        // Pass &contents
        Ok(results) => results,
        Err(e) => {
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

// run now takes a lifetime parameter 'a and borrows contents
fn run<'a>(config: Config, contents: &'a str) -> Result<Vec<&'a str>, Box<dyn Error>> {
    println!(
        "Searching for '{}' in file '{}'",
        config.query, config.file_path
    );
    // Removed: let contents = fs::read_to_string(config.file_path)?; // No longer read here

    let results = if config.ignore_case {
        println!("Ignoring case");
        // Pass the borrowed contents
        search_case_insensitive(&config.query, contents)
    } else {
        println!("Not ignoring case");
        // Pass the borrowed contents
        search(&config.query, contents)
    };

    Ok(results)
}

struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next(); // Skip the program name

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg)   => arg,
            None        => return Err("Didn't get a file path"),
        };
        //let ignore_case = args.get(3).is_some();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        println!("ignore_case: {}", ignore_case);
        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}
