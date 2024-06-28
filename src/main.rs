use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("I'm thinking of a number between 1 and 10.");
    println!("Try to guess the number I'm thinking of.");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_dev: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Invalid input. Please enter a number. {e}");
                continue;
            },
        };

        match guess_dev.cmp(&correct) {
            Ordering::Greater => {
                println!("Too high!");
            },
            Ordering::Less => {
                println!("Too low!");
            },
            Ordering::Equal => {
                println!("Congratulations! You guessed the correct number.");
                break;
            },
        };
    }

    // if guess_dev == correct_guess{
    //     println!("Congratulations! You guessed the correct number.");
    // } else if guess_dev < correct_guess {
    //     println!("Too low!");
    // } else if guess_dev > correct_guess {
    //     println!("Too high!");
    // }
}
