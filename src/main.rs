use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Welcome to my guessing game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("error");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("your guess is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too smol"),
            Ordering::Equal => {
                println!("you got it");
                break;
            }
        }
    }
}
