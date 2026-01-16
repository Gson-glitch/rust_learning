/*
  File Handling
    - File::create: Creates a new file (overwrites if exists).
    - File::open: Opens for reading only.
    - OpenOptions: Used for custom modes like "append".
    - io::Write / io::Read: Traits required to actually move data.
    - Rule of Thumb: Always use ? for anything that returns Result<()>. Think of it as unwrapping the contents
*/

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};

fn main() -> io::Result<()> {
    // Writing to a file
    let mut file: File = File::create("note.txt")?;
    file.write_all(b"Hello, Rust!\n")?;

    // Appending to a file
    let mut file: File = OpenOptions::new().append(true).open("note.txt")?;
    file.write_all(b"File handling in Rust")?;

    // Reading from a file
    let mut file: File = File::open("note.txt")?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents: {}", contents);

    // Reading from a file using buffered reader.
    // This reads from the file once and caches the contents to the RAM.
    // Unlike File::open() which has to open and read the file from disk each time,
    // Bufreader reads once and fetches from RAM for subsequent reads.
    let file: File = File::open("note.txt")?;
    let reader: BufReader<File> = BufReader::new(file);

    println!("\nReading files using buffered reader.");
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}
