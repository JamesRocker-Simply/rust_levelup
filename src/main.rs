// use srd is for the rust standard library
use std::io;  // io functionality (e.g. read and write)
use std::cmp::Ordering;  // result of a a comparison between two values
use rand::Rng;  //

extern crate rand;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut rng = rand::thread_rng();  // spawns the a sealed random number
    let secret_number = rng.gen_range(1, 101); // specifies the sealed range
    let mut guess = String::new();  // static mutable variable

    io::stdin().read_line(&mut guess)  // reads the line and assigns to the mutable guess var
        .ok()
        .expect("Failed to read line");  // in the event of a failure to read the line

    let guess: u32 = guess.trim().parse()  // Checks the guess is a 32 bit unsigned int
        .ok()
        .expect("Please type a number!");  // unwraps in the event of the above being not true

    println!("You guessed: {}", guess);
    println!("My random number: {}", secret_number);

    match guess.cmp(&secret_number) {  // compares the guess and secret_number var
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => println!("You win!"),
    }
}