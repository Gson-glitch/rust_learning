pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(query: String, filename: String) -> Self {
        Self { query, filename }
    }

    pub fn parse(&self) {
        let query: &String = &self.query;
        let filename: &String = &self.filename;
        println!("Query: {}\nFilename: {}", query, filename);
    }
}
