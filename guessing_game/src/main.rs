use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    // println!("Guess the number!");

    // println!("Please input your number: ");
    // let secret_number = rand::rng().random_range(1..=100);
    

    // loop {
    //     let mut guess: String = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = guess.trim().parse().expect("Please type a number!");


    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big"),
    //         Ordering::Equal => {
    //             println!("You win");
    //             break;
    //         }
    //     }
    // }

    let secret_number = rand::rng().random_range(1..=100);
    let cnt = 3;
    loop {
        let mut _in_number = String::new();
        io::stdin()
            .read_line(&mut _in_number)
            .expect("Failed to read line");

        let _in_number: u32 = match _in_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match _in_number.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }

        println!("You guessed: {_in_number}")
    }

    
}
