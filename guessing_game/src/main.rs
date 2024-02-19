pub mod example_debug;
pub mod example_type;
mod example_struct;
mod example_lifecycle;
use std::{cmp::Ordering, io};
use rand::Rng;
pub mod example_common;
fn main() {
    example_common::debug_pre("Guess the number(1-100) for 9 times");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);
    let mut count = 9;
    loop {
        if count == 0 {
            println!("You failed the right number is {}", secret_number);
            break;
        }

        let mut guess = String::new();
        println!("Remining {} times", count);
        println!("Please input your guess.");

        io::stdin().read_line(&mut guess).expect("Fail to read line");
    
        let guess: u32 = match  guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        count -= 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
    example_common::debug_post();
}
