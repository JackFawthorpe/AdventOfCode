extern crate regex;

use regex::Regex;

pub fn solve(input: String) -> String {
    return second_star(input);
}

// Converts a string to a vector of chars (Easier to process char by char)
fn string_to_char_vec(input: &str)-> Vec<char> {
    let mut chars: Vec<char> = Vec::new();
    for c in input.chars() {
        chars.push(c);
    }
    return chars;
}

/// Converts a char array into a string from the start index to the end index (inclusive)
fn char_vec_to_string(chars: &Vec<char>, start_index: usize, end_index: usize) -> String {
    chars.into_iter()
    .skip(start_index)
    .take(end_index - start_index + 1)
    .collect()
}

// Takes a string reference and returns the number 1-9 that the string starts with (returns zero if no digit present)
fn to_number(section: &str) -> u32 {
    
    let mut number_regexs: Vec<Regex> = Vec::new();
    number_regexs.push(Regex::new(r"^one").unwrap());
    number_regexs.push(Regex::new(r"^two").unwrap());
    number_regexs.push(Regex::new(r"^three*").unwrap());
    number_regexs.push(Regex::new(r"^four").unwrap());
    number_regexs.push(Regex::new(r"^five").unwrap());
    number_regexs.push(Regex::new(r"^six").unwrap());
    number_regexs.push(Regex::new(r"^seven").unwrap());
    number_regexs.push(Regex::new(r"^eight").unwrap());
    number_regexs.push(Regex::new(r"^nine").unwrap());
    number_regexs.push(Regex::new(r"^ten").unwrap());

    for (index, regex) in number_regexs.iter().enumerate() {
        if regex.is_match(section) {
            return (index + 1) as u32;
        }
    }

    return 0;
}

// Finds the first / last digit in a &vector<char> of either digit form or spelled out (etc. one).
// Ordering is dependent on from_start boolean
fn find_digit(chars: &Vec<char>, from_start: bool) -> u32 {
    let iterator: Box<dyn Iterator<Item = &char>>;
    if from_start {
        iterator = Box::new(chars.iter());
    } else {
        iterator = Box::new(chars.iter().rev());
    }

    for (i, char) in iterator.enumerate() {
        let index: usize;
        if from_start {
            index = i;
        } else {
            index = chars.len() - 1 - i;
        }
        if char.is_digit(10) {
            return char.to_digit(10).unwrap_or(0);
        }
        let section: String = char_vec_to_string(&chars, index, (index + 5).min(chars.len() - 1));
        let result: u32 = to_number(&section);
        if result != 0 {return result;}
    }
    return 0; // Won't happen
}

// Takes a line of the input and converts it into a two digit number of the first and last number of the line
fn process_line(input: &str) -> u32 {
    let chars: Vec<char> = string_to_char_vec(input);
    return find_digit(&chars, true) * 10 + find_digit(&chars, false);
}


// Second star involves taking a list of strings that include numbers (1 or one) (1-9) and summing the results
fn second_star(input: String) -> String {
    let mut sum: u32 = 0;
    for (index, line) in input.lines().enumerate() {
        let result: u32 = process_line(line);
        println!("Result for line {} is {}", index, result);
        sum += result;
    }
    return sum.to_string();
}

// First Star

// First star involves taking a list of strings and summing the results when you take the first and last digit of each string
#[allow(dead_code)]
fn first_star(input: String) -> String {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let digits: Vec<u32> = get_digits(line);
        sum += digits[0] * 10 + digits[digits.len() - 1];
    }
    return sum.to_string();
}

// Returns the list of digits present in a string
fn get_digits(input: &str) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    for c in (*input).chars() {
        if c.is_digit(10) {
            digits.push(c.to_digit(10).unwrap_or(0));
        }
    }
    return digits;
}