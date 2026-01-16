use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn parse(args: &[String]) -> Result<String, io::Error> {
        if args.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Not enough arguments! Usage: cargo run --bin cli_parser <query> <file>",
            ));
        }

        let config: Config = Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        };

        let file: File = File::open(&config.filename)?;
        let mut found: bool = false;
        let mut found_line: String = String::new();
        let reader: BufReader<File> = BufReader::new(file);
        for line in reader.lines() {
            let line: String = line?;
            if line.contains(config.query.as_str()) {
                found = true;
                found_line = line;
            }
        }
        // let mut contents: String = String::new();
        // file.read_to_string(&mut contents)?;
        // let found: bool = contents.contains(config.query.as_str());

        match found {
            true => Ok(format!(
                "Query: {} found in line: {}",
                config.query, found_line
            )),
            false => Ok("Not found".to_string()),
        }
    }
}
