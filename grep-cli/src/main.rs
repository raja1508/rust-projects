use std::{env, process};
use grep_cli::{search_query_in_file, Config};  


fn main() {
    eprintln!("Usage fomat ----> cargo run <query> <file_path>");

    let args: Vec<String> = env::args().into_iter().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);  
    }); 

    eprintln!("Query: {:?}", config.query);
    eprintln!("Search file: {:?}", config.file_path); 

    let res = search_query_in_file(&config.query, &config.file_path, config.ignore_case); 
    match res {
        Ok(value) => {
            println!("{:?}", value);
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);  
        } 
    }

}
