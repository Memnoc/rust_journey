use std::{cmp::Ordering, io};

use rand::Rng; // prelude

fn main() {
    println!("== Guessing Game ==");

    let secret_number = rand::thread_rng().gen_range(1..=100); // generating a random number from 1
                                                               // to 100
    println!("Secret number: {}", secret_number);

    println!("== Please input your guess ==");

    let mut guess = String::new(); // new string

    io::stdin()
        .read_line(&mut guess) // intake user input
        .expect("Failed to read line"); // handling failures

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
}
