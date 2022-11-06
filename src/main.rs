extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::{Rng, thread_rng};


fn main() {
    println!("Guess the number!");

    let mut thread_random = thread_rng();
    let secret_number = thread_random.gen_range(0..100);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let number_guess: u32 = guess.trim().parse()
        .expect("Please type a number");

    println!("You guessed: {}", number_guess);

    match number_guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win"),
    }
}
