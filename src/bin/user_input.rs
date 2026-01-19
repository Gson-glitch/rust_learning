use std::io;

fn main() {
    println!("Please enter your input.");
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");
    println!("Your input was: {user_input}");
}
