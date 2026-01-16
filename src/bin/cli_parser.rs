use rust_learning::cli::parser::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", Config::parse(&args));
}
