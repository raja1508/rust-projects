use std::io::{Read, Write};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::{io, println}; 
use std::fs::File; 
use std::time::Instant; 
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter}; 

// hello.txt
// Zip file: compr_hello
// Source file size: 31
// Zip file size: 149
// Time taken for compression 2.138049ms

// atomic-habits.pdf
// 1) Deflated
// Source file size: 6667711
// Zip file size: 6359125
// Time taken for compression 8.770889608s
// 3) Zstd
// Source file size: 6667711
// Zip file size: 5932117
// Time taken for compression 14.368509909s
fn main() {
    let mut source_file: String= String::new(); 
    let mut zip_file: String = String::new(); 
    let mut compression_choice: String = String::new(); 

    println!("Enter the path of the source file to compress: ");
    io::stdin().read_line(&mut source_file).unwrap();
    let source = source_file.trim();

    println!("Enter the destination path of the zip file: "); 
    io::stdin().read_line(&mut zip_file).unwrap();
    let zip_file = zip_file.trim();  

    println!("Choose a compression method: "); 
    println!("1)Deflated 2)Stored 3)Bzip2 4)Zstd"); 
    io::stdin().read_line(&mut compression_choice).unwrap(); 
    let compression_choice = compression_choice.trim(); 

    let compression_method = match compression_choice {
        "1" => CompressionMethod::Deflated ,
        "2" => CompressionMethod::Stored, 
        "3" => CompressionMethod::Bzip2, 
        "4" => CompressionMethod::Zstd,
        _ => CompressionMethod::Deflated,
    }; 

    let start = Instant::now(); 

    
    println!("Source file: {}", source); 
    println!("Zip file: {}", zip_file); 
    println!("Compression method: {}", compression_method); 
    
    compress_to_zip(source, zip_file, compression_method).unwrap();

    let elapse_time = start.elapsed();
    println!("Time taken for compression {:?}", elapse_time); 
    
}


fn compress_to_zip(
    source: &str,
    zip_file: &str,
    compression_method: CompressionMethod
) -> Result<(), Box<dyn std::error::Error>> {

    let mut source_handler = File::open(source)?; 
    let mut buffer = Vec::new(); 

    //  CONVERTING THE BUFFER INTO UTF-8 ENCODING
    // let mut content = String::new();
    // content = String::from_utf8(buffer.clone())?; 
    // println!("Source file content: {:?}", content); 
    

    // RESETING THE CURSOR POSITION FOR RE-READ
    // source_handler.seek(io::SeekFrom::Start(0))?; 
    // source_handler.read_to_end(&mut content)?;  

    source_handler.read_to_end(&mut buffer)?;
    println!("Source file bytes: {:?}", buffer);    
    
    let zip_path = Path::new(zip_file);
    let zip_handler = File::create(zip_path)?;
    
    let mut zip = ZipWriter::new(&zip_handler); 
    zip.start_file(source, FileOptions::<()>::default().compression_method(compression_method))?;
    zip.write_all(&buffer)?; 
    zip.finish()?; 
    
    let source_size = source_handler.metadata()?.size(); 
    println!("Source file size: {:?}", source_size); 
    
    let zip_size = zip_handler.metadata()?.size(); 
    println!("Zip file size: {:?}", zip_size); 

    Ok(())
}