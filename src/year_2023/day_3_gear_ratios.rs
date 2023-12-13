pub fn solve(input: String) -> String {
    return first_star(input);
}

fn first_star(input: String) -> String {
    let lines: Vec<Vec<char>> = massage_input(input);
    let grid_size: (usize, usize) = (lines[0].len(), lines.len());
    let mut bitmask = vec![false; grid_size.0 * grid_size.1];
    let mut sum: u32 = 0;

    for i in 0..grid_size.0 {
        for j in 0..grid_size.1 {
            let char = lines[i][j];
            if !(char == '.') && !char.is_numeric() {
                let position = (i, j);
                test(&lines, &mut bitmask, &mut sum, position);
            }
        }
    }

    sum.to_string()
}

fn test(lines: &Vec<Vec<char>>, bitmask: &mut Vec<bool>, sum: &mut u32, position: (usize, usize)) {
    let grid_size: (usize, usize) = (lines[0].len(), lines.len());
    for i in position.0 - 1..=position.0 + 1 {
        for j in position.1 - 1..=position.1 + 1 {
            if lines[i][j].is_numeric() && !bitmask[i * grid_size.1 + j] {
                *sum += get_number(&lines[i], (i, j), bitmask);
            }
        }
    }
}

fn get_number(line: &Vec<char>, position: (usize, usize), bitmask: &mut Vec<bool>) -> u32 {
    let line_size = line.len();
    let mut left = position.1;
    let mut right = position.1;

    while line[left].is_numeric() {
        bitmask[position.0 * line_size + left] = true;
        left -= 1;
    }

    while line[right].is_numeric() {
        bitmask[position.0 * line_size + right] = true;
        right += 1;
    }

    let mut output = 0;
    for i in left + 1..right {
        output *= 10;
        output += line[i].to_digit(10).unwrap();
    }

    output
}

fn massage_input(input: String) -> Vec<Vec<char>> {
    let mut lines: Vec<String> = input.lines().map(|str| str.to_string()).collect();

    let grid_size: (usize, usize) = (lines[0].len(), lines.len());

    let y_padding = ".".repeat(grid_size.1);
    lines.insert(0, y_padding.clone());
    lines.push(y_padding.clone());

    let mut char_array: Vec<Vec<char>> = Vec::new();

    for line in lines.iter() {
        let mut chars : Vec<char> = line.chars().collect();
        chars.push('.');
        chars.insert(0, '.');
        char_array.push(chars);
    }

    char_array
}