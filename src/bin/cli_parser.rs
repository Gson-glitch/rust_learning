use rust_learning::cli::parser::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(args[1].clone(), args[2].clone());
    config.parse();
}
