use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("How many random numbers do you want to guess");
    let mut how_many_guess = String::new();

    io::stdin()
        .read_line(&mut how_many_guess)
        .expect("Error reading input");

    let num_guesses: u8 = how_many_guess.trim().parse().expect("Error parsing");

    let mut correct = Vec::new();

    for _i in 0..num_guesses {
        correct.push(rand::thread_rng().gen_range(1..=9999999));
    }

    println!("{correct:?}");

    let mut attempts = 0;

    while attempts < num_guesses {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_dev: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Invalid input. Please enter a number. {e}");
                continue;
            }
        };

        match guess_dev.cmp(&correct[attempts as usize]) {
            Ordering::Greater => {
                println!("Too high!");
            }
            Ordering::Less => {
                println!("Too low!");
            }
            Ordering::Equal => {
                println!("Congratulations! You guessed the correct number.");
                attempts += 1;
                if attempts < num_guesses{
                    println!("Let's now try the next number! \n");
                }
            }
        };
    }

    println!("\nThank you for playing the game! The correct numbers where:");

    for item in correct{
        println!("{item}");
    }

    // if guess_dev == correct_guess{
    //     println!("Congratulations! You guessed the correct number.");
    // } else if guess_dev < correct_guess {
    //     println!("Too low!");
    // } else if guess_dev > correct_guess {
    //     println!("Too high!");
    // }
}
