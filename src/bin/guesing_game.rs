use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::rng().random_range(1..=10);

    loop {
        println!("Please enter a number.");

        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                return;
            }
        };
        match user_input.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
