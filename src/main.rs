use colored::*;
use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self, Write},
    process::exit,
};

fn main() {
    clear_output();
    println!("Welcome!");
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("{}", "Guess the number!".purple());

        loop {
            println!("{}", "Please input your guess:".bright_blue());

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line!");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("The input `{}` is invalid", guess.trim().red());
                    continue;
                }
            };

            clear_output();

            println!("You guessed: {}", guess);

            let mut won_the_game = false;
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Too small!".yellow()),
                Ordering::Greater => println!("{}", "Too big!".magenta()),
                Ordering::Equal => {
                    won_the_game = true;
                    println!("{}", "You win the game!".bright_green())
                }
            }

            if !won_the_game {
                continue;
            }

            let mut should_continue = String::new();
            println!("{}", "You want to play again? [Y/n]".bright_cyan());

            io::stdin()
                .read_line(&mut should_continue)
                .expect("Failed to read the response!");

            match should_continue.trim().to_uppercase().as_str() {
                "Y" => {
                    clear_output();
                    println!("{}", "Lets play again!".bright_magenta());
                    break;
                }
                _ => {
                    println!("{}", "See ya!".bright_white());
                    exit(1);
                }
            }
        }
    }
}

fn clear_output() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}
