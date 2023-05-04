use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing number game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("secret number is {secret_number}");

    loop {
        println!("Input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        println!("read guess number: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("too great"),
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        };
    }
}
