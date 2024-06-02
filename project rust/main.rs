use std::io;

fn main() {
    println!("Enter a number to calculate its factorial:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    
    let num: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };
    
    println!("Factorial of {} is {}", num, factorial(num));
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
