
fn get_digits(input: &str) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    for c in (*input).chars() {
        if c.is_digit(10) {
            digits.push(c.to_digit(10).unwrap_or(0));
        }
    }
    return digits;
}

pub fn solve(input: String) -> String {
    let mut sum: u32 = 0;
    let lines = input.split('\n');
    for line in lines {
        let digits: Vec<u32> = get_digits(line);
        sum += digits[0] * 10 + digits[digits.len() - 1];
    }
        
    return sum.to_string();
}