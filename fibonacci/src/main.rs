use std::io; 

fn fibonacci(nth: u32, mut first: u32, mut second: u32) -> u32 {
    let mut fibo_series = Vec::new(); 
    fibo_series.push(first); 
    fibo_series.push(second); 

    if nth == 1 && nth == 2 {
        return first; 
    }

    for _i in 2..nth {
        let temp = first; 
        first = second; 
        second += temp; 
        fibo_series.push(second); 
    
    }

    println!("Fibonacci series = {:?}", fibo_series); 

    return second; 
    
}

fn main() {
    let  first: u32 = 1 ; 
    let  second: u32 = 1 ; 

    let mut input = String::new(); 

    io::stdin().read_line(&mut input).unwrap(); 

    let nth : u32 = input.trim().parse().expect("Number is expected");
    let number = fibonacci(nth, first, second); 
    println!("The nth number in fibonacci series = {}", number); 

    
}
