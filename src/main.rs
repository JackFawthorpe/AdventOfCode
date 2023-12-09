use std::{fs::{self}, io};

static INPUT_PATH: &str = "./input.txt";

fn main() {
    match get_input() {
        Ok(input) => {solve(input)},
        Err(error) => eprintln!("Couldn't open input file: {}", error),
    }
}

fn get_input() -> Result<String, io::Error> {
    let input: String = fs::read_to_string(INPUT_PATH)?;
    Ok(input)
}

fn solve(input: String) {
    println!("Received input {}", input);
}