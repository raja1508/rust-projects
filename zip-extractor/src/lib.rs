use std::{eprintln, fs::{self, File}, io::{self}, path::Path};

use zip::{ZipArchive};

#[derive(Debug)]
pub struct  ZipConfig {
    pub zip_file: String,
    pub output_folder: String
} 

impl ZipConfig {
    pub fn build(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args")
        } 

        let zip_config = ZipConfig {
           zip_file : args.iter().nth(1).unwrap().clone(),
           output_folder: args.iter().nth(2).unwrap().clone()
        }; 

        Ok(zip_config)

    }
}

pub fn zip_extraction(zip_file: &String, output_folder: &String) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::open(zip_file)? ; 
    let mut zip_archive =  ZipArchive::new(zip_file)?; 

    for i in 0..zip_archive.len() {
        let mut entry = zip_archive.by_index(i)?; 

        // logging some entry details
        let name = entry.name().to_owned(); 
        let if_folder = entry.is_file(); 
        let file_or_folder = if !if_folder { "Folder" } else { "File" }; 
        eprintln!("Entry {}: \n Name: {} \n File/Folder: {}", i, name, file_or_folder ); 

        // creating output folder and files 
        let output_folder_path = Path::new(output_folder);
        if !output_folder_path.exists(){
            std::fs::create_dir(output_folder)?; 
        } 
        
        let output_path = output_folder_path.join(&name); 
        
        if name.ends_with("/") {
            fs::create_dir_all(output_path)?; 
        } else {
            let mut output = File::create(output_path)?; 
            io::copy(&mut entry, &mut output)?; 
        }


        // extracting the contents of zip file

        // let mut buffer = Vec::new(); 
        // entry.read_to_end(&mut buffer)?; 
        // let content = String::from_utf8(buffer)?; 
        // println!("Contents: {}", content); 
    }

    Ok(())

}