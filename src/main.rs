#[path = "year_2023/day_6_wait_for_it.rs"]
pub mod problem;

use std::fs;
static INPUT_PATH: &str = "./input.txt";
static OUTPUT_PATH: &str = "./output.txt";

fn get_input() -> Result<String, std::io::Error> {
    let input: String = fs::read_to_string(INPUT_PATH)?;
    Ok(input)
}

fn save_output(contents: String) -> Result<(), std::io::Error> {
    fs::write(OUTPUT_PATH, contents)?;
    Ok(())
}


fn run() -> Result<(), std::io::Error> {
    let input: String = get_input()?;
    let output: String = problem::solve(input);
    save_output(output)?;
    Ok(())
}

fn main() {
    match run() {
        Err(err) => println!("Failed to solve: {}", err),
        Ok(_) => println!("Solved, exitting successfully"),
    }
}