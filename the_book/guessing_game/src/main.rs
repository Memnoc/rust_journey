use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("== Guessing Game ==");

    //NOTE: generating a random num from 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //NOTE: comparing the guess and the number
    // adding a loop to give users more chances to guess
    loop {
        println!("== Please input your guess ==");
        let mut guess = String::new();

        //NOTE: intake user input while handling failures
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //NOTE: converting String to num u32
        // also parsing it to remove spaces
        // error handling using parse Result enum
        // Err(_) is a catch-all value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // NOTE: another use of match for comparison
        // notice the use of break opening a block on one arm
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
