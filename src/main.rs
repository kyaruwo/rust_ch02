use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Rust Number Guessing Game!");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u8 = match guess.trim().parse() {
            Ok(numb) => numb,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("chisai"),
            Ordering::Greater => println!("dekai"),
            Ordering::Equal => {
                println!("sugoi!");
                break;
            }
        }
    }
}
