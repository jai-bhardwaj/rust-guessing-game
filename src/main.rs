extern crate rand;
extern crate colored;

use rand::Rng;
use colored::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("{}","----------Guess the number!----------".blue());

    let random_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("{}","------Please input your guess:-------".blue());
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}","?????????You entered wrong input".red());
                continue
            }
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("             {}","Too small!".red()),
            Ordering::Greater => println!("             {}","Too big!".red()),
            Ordering::Equal => {
                println!("             {}","You win!!!".green());
                break;
            },
        }
    }

}
