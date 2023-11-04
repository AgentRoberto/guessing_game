use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let mut input = String::new();

    println!("Please input your guess.");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    // Parse the string to an appropriate number type
    let guess: Result<u8, _> = input.trim().parse();
    match guess {
        Ok(number) => {
            let mut rng = rand::thread_rng();
            let random_number: u8 = rng.gen();

            if random_number == number {
                println!("Your guess was correct! The number is: {}", number);
            }
            // Do something with the parsed number
            println!("Guessed number: {}", number);
            println!("Correct number: {}", random_number);

        }
        Err(e) => {
            println!("Failed to parse input as a number: {}", e);
        }
    }
    
}
