use std::io;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(0..10);
    println!("Guess the number!");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line.");
        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                println!("You won!");
                break;
            }
            std::cmp::Ordering::Less => println!("Too small... try again."),
            std::cmp::Ordering::Greater => println!("Too big... try again"),
        }
    }
}
