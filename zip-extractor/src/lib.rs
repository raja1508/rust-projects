use std::{eprintln, fs::File, io::{self, Read}, println};

use zip::{ZipArchive};

#[derive(Debug)]
pub struct  ZipConfig {
    pub zip_file: String,
    pub output_path: String
} 

impl ZipConfig {
    pub fn build(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args")
        } 

        let zip_config = ZipConfig {
           zip_file : args.iter().nth(1).unwrap().clone(),
           output_path: args.iter().nth(2).unwrap().clone()
        }; 

        Ok(zip_config)

    }
}

pub fn zip_extraction(zip_file: &String, _output_path: &String) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::open(zip_file)? ; 
    let mut zip_archive =  ZipArchive::new(zip_file)?; 

    for i in 0..zip_archive.len() {
        let mut entry = zip_archive.by_index(i)?; 

        // logging some entry details
        let name = entry.name(); 
        let if_folder = entry.is_file(); 
        let file_or_folder = if !if_folder { "Folder" } else { "File" }; 
        eprintln!("Entry {}: \n Name: {} \n File/Folder: {}", i, name, file_or_folder ); 

        // creating output files 
        let mut ouput = File::create(_output_path)?; 


        // extracting the contents of zip file
        io::copy(&mut entry, &mut ouput)?; 

        let mut buffer = Vec::new(); 
        entry.read_to_end(&mut buffer)?; 
        let content = String::from_utf8(buffer)?; 
        println!("Contents: {}", content); 
    }

    Ok(())

}