use std::io;
use rand::Rng;

fn main() {
    let mut is_correct: bool = false;
    while !is_correct {
        println!("Guess the number!");

        let mut input = String::new();
        println!("Please input your guess.");
        
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let guess: Result<u8, _> = input.trim().parse();

        // Parse the string to an appropriate number type
        match guess {
            Ok(number) => {
                let mut rng = rand::thread_rng();
                let random_number: u8 = rng.gen();
                if random_number == number {
                    is_correct = true;
                    println!("Your guess was correct! The number is: {}", number);
                }
                // Do something with the parsed number
                println!("Guessed number: {}", number);
                println!("Try Again ");
            }
            Err(e) => {
                println!("Failed to parse input as a number: {}", e);
            }
        }
    }
    
}
