#![allow(non_snake_case)]
use std::io;
use rand::Rng;
pub fn run() {
    println!("Guess the number between 1 and 6");
    println!("Please input your guess:");
    let secret_number = rand::thread_rng().gen_range(1..7);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess = guess.trim().parse::<i32>().unwrap(); 
    if guess == secret_number {
        println!("You guessed the correct number");
    }
    else {
        println!("You guessed the wrong number");
        println!("The correct number was {}",secret_number);
    }


}