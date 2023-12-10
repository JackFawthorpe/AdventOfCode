extern crate regex;

use regex::Regex;

// Gets all of the lines that are in a given input
pub fn get_lines(input: &str) -> Vec<&str> {
    return input.split("\r\n").collect();
}

// Takes a string and returns a vector of all the integers in the string
#[allow(dead_code)]
pub fn get_integers_from_input(input: &str) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    let regex = Regex::new(r"\\d+").unwrap();

    for found in regex.find_iter(input) {
        numbers.push(found.as_str().parse().unwrap());
    }

    return numbers;
}