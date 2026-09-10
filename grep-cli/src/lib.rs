use std::{env, fs, path::Path};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments"); 
        }

        let config = Self {
            query : args.iter().nth(1).unwrap().clone(),
            file_path : args.iter().nth(2).unwrap().clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok()
        }; 

        Ok(config)
    }
}


pub fn search_query_in_file(query: &String, file_path: &String, ignore_case: bool) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let search_file_path = Path::new(file_path); 
    let content = fs::read_to_string(search_file_path)?; 

    let mut res = Vec::<String>::new(); 

    if ignore_case {
        for line in content.lines() { 
            if line.to_lowercase().contains(&query.to_lowercase()) {
                res.push(line.to_string());
            }
        }
    } else {
        for line in content.lines() { 
            if line.contains(query) {
                res.push(line.to_string());
            }
        }
    }
    Ok(res)
}