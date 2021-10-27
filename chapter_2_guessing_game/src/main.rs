use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let num = rand::thread_rng().gen_range(1..100);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let mut guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => { println!("Not a valid number"); continue; },
        };

        match guess.cmp(&num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!"); break; },
        }
    }
}
