use std::fs;

fn main() {
    println!("Reading file...");

    match fs::read_to_string("data.txt") {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}