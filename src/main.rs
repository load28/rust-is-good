#![allow(dead_code)]

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::{Rng, thread_rng};

fn main() {
    // run_guessing_game();
    run_variable();
}

fn run_variable() {
    let mut x = 5;
    println!("x - {}", x);
    x = 10;
    println!("x - {}", x);
}



fn run_guessing_game() {
    println!("Guess the number!");

    let mut thread_random = thread_rng();
    let secret_number = thread_random.gen_range(0..100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let number_guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Please input number");
                continue;
            },
        };

        println!("You guessed: {}", number_guess);

        match number_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
