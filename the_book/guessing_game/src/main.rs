use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("== Guessing Game ==");

    //NOTE: generating a random num from 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number: {}", secret_number);

    println!("== Please input your guess ==");
    let mut guess = String::new();

    //NOTE: intake user input while handling failures
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //NOTE: converting guess string to num with parse()
    // parse also returns Result type so we can do error handling
    // this is also demonstrating "shadowing"
    // notice that we need to specify the number type
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {guess}");

    //NOTE: comparing the guess and the number
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
}
