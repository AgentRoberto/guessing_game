use std::io;
use rand::Rng;
use std::process::exit;

fn main() {
    let mut is_correct: bool = false;
    println!("Guess a number between 1 and 10!");

    while !is_correct {
        let mut input = String::new();
        println!("Please input your guess.");

        io::stdin().read_line(&mut input).expect("Failed to read input");
        let guess: Result<u8, _> = input.trim().parse();

        // Parse the string to an appropriate number type
        match guess {
            Ok(number) => {
                let mut rng = rand::thread_rng();
                let random_number: u8 = rng.gen_range(0..10);
                if random_number == number {
                    is_correct = true;
                    println!("Your guess was correct! The number is: {}", number);
                    exit(1);
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
