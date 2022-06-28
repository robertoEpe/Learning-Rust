use std::env;
use std::fs;

struct Config {
    file : String,
    query: String,
}

impl  Config {
    fn new(file: String, query: String) -> Self {
        Config { file, query }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:#?}", args);

    let filename = args.get(1);
    let query = args.get(2);

    match filename {
        Some(filename) => {
            match query {
                Some(query) => {
                    process(Config::new(filename.clone(), query.clone()));
                },
                None => {
                    println!("No query was provided");
                }
            }
        },
        None => {
            println!("No filename was provided");
        }
    }

}


fn process(config: Config) {
    println!("archivo {}", config.file);
    println!("buscar {}", config.query);

    match fs::read_to_string(config.file) {
        Ok(contents) => {
            let lines_found = search(&config.query, &contents);

            for line in lines_found {
                println!("{line}");
            }

        },
        Err(_) => {
            println!("Failed to read file");
            return
        } 
    } 
}

fn search<'a>(query: &str, contents:  &'a str) -> Vec<  &'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.trim());
        }
    }

    result

}