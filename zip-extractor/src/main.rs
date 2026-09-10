use std::{env, eprintln, process}; 
use zip_extractor::{ZipConfig, zip_extraction}; 

fn main() {
    let args : Vec<String> = env::args().into_iter().collect(); 
    let zip_conf = ZipConfig::build(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err); 
        process::exit(1); 
    });  

    eprintln!("{:?}", zip_conf); 
    zip_extraction(&zip_conf.zip_file, &zip_conf.output_path).unwrap(); 
    
}
