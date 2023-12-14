use std::collections::HashSet;

pub fn solve(input: String) -> String {
    second_star(input)
}

// Returns the amount of winning numbers the ticket contains
fn get_matches_on_line(line: String) -> u32 {
    let mut all_numbers = line.split(':').nth(1).unwrap().split('|');
        
        // Create a set of the winning numbers
        let mut winning_numbers: HashSet<u32> = HashSet::new();
        let winning_numbers_list = all_numbers.nth(0).unwrap();
        winning_numbers_list.trim().split_whitespace().for_each(|num| { 
            winning_numbers.insert(num.parse::<u32>().unwrap()); });
        
        // Iterate over the ticket's numbers and increment
        // the exp each match with the winning set
        let mut matches: u32 = 0;
        let ticket_numbers = all_numbers.nth(0).unwrap();
        ticket_numbers
        .trim()
        .split_whitespace()
        .for_each(|num_str| {
            let num = num_str.parse::<u32>().unwrap();
            if winning_numbers.contains(&num) {
                matches += 1;
            }
        });

        matches
}

fn second_star(input: String) -> String {
    let mut sum: u32 = 0;
    // Structure to represent the amount of the next 10 tickets we have
    // (capped at 10 as current ticket can only influence the count of the next 10)
    let mut scratches: Vec<u32> = vec!(1, 1, 1, 1, 1, 1, 1, 1, 1, 1);
    let lines: Vec<String> = input.lines().map(|str| str.to_string()).collect();

    for line in lines {     
        let current_ticket_count = scratches[0];
        sum += current_ticket_count;
        for index in 1..10 {
            scratches[index - 1] = scratches[index];
        }
        scratches[9] = 1;
        let matches = get_matches_on_line(line);
        for index in 0..(matches as usize) {
            scratches[index] += current_ticket_count;
        }
    }

    sum.to_string()
}

#[allow(dead_code)]
fn first_star(input: String) -> String {
    let lines: Vec<String> = input.lines().map(|str| str.to_string()).collect();
    let mut sum: u32 = 0;
    for line in lines {
        // If ticket had winning numbers add to score
        let exponent = get_matches_on_line(line) as i32 - 1;
        if  exponent >= 0 {
            sum += 2_u32.pow(exponent.try_into().unwrap())
        }
    }

    sum.to_string()
}