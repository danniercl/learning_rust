// 2. Let extern a Rust library crate to randomize numbers
extern crate rand;

use std::io;
// 2. From rand, we use Rng;
use rand::Rng;
// 3. Using the standard library for compate
use std::cmp::Ordering;

fn main() {
    // 1.These are simple prints. Easy!
    println!("Guess the number from 0 to 100!");

    println!("Please input your guess. [0 -100]");

    // 2. Generate the random number, it's not necessary
    // 2. to do a mutable variable because it's gonna be
    // 2. use once.
    // 2. Look at the range and the inclusiveness ->[a,b[
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // 2. Just debugging: printing the random number.
    println!("The secret number is: {}", secret_number);

    // 1.Declaring a variable with mutability.
    // 1.The type is a string, look at the instance:
    // 1.Yeah, is a "new String".
    let mut guess = String::new();

    // 1.First function from "io" library, it waits 
    // 1.the value from console.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // 4. We need to cast the input from String to u32
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    // 3. Ok, it has sense, but is a bit complicated.
    // 3. This is like a swith-case.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    // 1. Ok, apparently the user always wins.
    println!("You guessed: {}", guess);
}
