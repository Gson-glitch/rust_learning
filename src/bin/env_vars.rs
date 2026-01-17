use dotenv::dotenv;
use std::io::ErrorKind;
use std::{env, io};

fn main() -> Result<(), io::Error> {
    dotenv().ok();
    let api_key: Result<String, env::VarError> = env::var("API_KEY");

    match api_key {
        Ok(val) => {
            println!("API_KEY: {}", val);
            Ok(())
        }
        Err(_e) => Err(io::Error::new(ErrorKind::NotFound, "API_KEY not found")),
    }
}
