use std::{cmp::Ordering, io}; 
use rand::Rng; 

fn main() {
    println!("Guess game");
    println!("Please enter your guess");

    let secret_number= rand::thread_rng().gen_range(1..=100) as u32; 


    loop {
        
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        
        let guess: u32 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        }; 
        
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You won"); 
                break ;
            } 
        }; 
    }; 
     println!("SECRET NUMBER = {}", secret_number);

}
