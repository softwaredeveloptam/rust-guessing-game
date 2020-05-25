use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {

    let mut rng = thread_rng();
    let random_num = rng.gen_range(0, 10) + 1;

    loop {
        println!("Please enter a number between 1 to 10.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input"); 
                continue 
            },
        };

        match guess.cmp(&random_num) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
