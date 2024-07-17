use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io::stdin, u32};
use colored::*;

fn main() {
    let n = thread_rng().gen_range(1..=100);
    // println!("The number is: {n}");

    println!("Welcome to guessing game!");

    // sleeker way of writing `while true` in Rust
    loop {
        println!("Enter your guess");
        let mut guess = String::new();
        // assigning some value to `guess` to see the effect of
        // appending that happens in `read_line`
        // guess = "hi".to_string();
        stdin().read_line(&mut guess).expect("system error");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid num, try again");
                continue;
            }
        };

        // println!("You guessed: {guess}");

        match guess.cmp(&n) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "Equal! Exiting...".green());
                break;
            }
        }
    }
}
