use crate::utils::string_utils::get_lines;

// Struct to represent what is pulled out of a bag
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

// Functionality associated with the bag
impl Bag {
    
    // Function to check whether the current bag is smaller in each aspect to the other bag
    fn is_smaller(&mut self, other: &Bag) -> bool {
        if other.blue < self.blue {
            return false;
        }
        if other.green < self.green {
            return false;
        }
        if other.red < self.red {
            return false;
        }
        return true;
    }
}

// Creates an empty bag
fn init_bag() -> Bag {
    return Bag {
        red: 0,
        green: 0,
        blue: 0,
    }
}

pub fn solve(input: String) -> String {
    return second_star(&get_lines(&input)).to_string();
}

// Sums the products of the smallest possible bags of each game
fn second_star(lines: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;

    for line in lines {
        let line_bag: Bag = get_min_bag(line);
        sum += line_bag.blue * line_bag.red * line_bag.green;
    }

    return sum;
}


// Sums the game numbers of each game that requires a bag smaller that the test bag
#[allow(dead_code)]
fn first_star(lines: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;

    let test_bag = Bag {
        red: 12,
        green: 13,
        blue: 14
    };

    for (index, line) in lines.iter().enumerate() {
        let mut line_bag: Bag = get_min_bag(line);
        if line_bag.is_smaller(&test_bag) {
            sum += (1 + index) as u32;
        }
    }
    return sum;
}

// Returns the smallest bag needed for the game on a line
fn get_min_bag(line: &str) -> Bag {
    let draws: Vec<Bag> = get_draws(line);

    let mut min_bag: Bag = init_bag();

    for draw in draws {
        min_bag.blue = min_bag.blue.max(draw.blue);
        min_bag.green = min_bag.green.max(draw.green);
        min_bag.red = min_bag.red.max(draw.red);
    }

    return min_bag;
}

// Returns the different draws that are seen within a game
fn get_draws(line: &str) -> Vec<Bag> {
    let mut bags: Vec<Bag> = Vec::new();
    
    let without_game = line.split(':').nth(1).unwrap();
    let draws: Vec<&str> = without_game.split(';').collect();
    for draw in draws {
        bags.push(draw_to_bag(draw));
    }
    
    return bags;
}


// Converts a singular draw from the bag into a bag
fn draw_to_bag(draw: &str) -> Bag {
    let mut bag: Bag = init_bag();

    let cubes : Vec<&str> = draw.split_ascii_whitespace().collect();

    for (index, cube_count) in cubes.iter().enumerate().step_by(2) {
        match cubes[index + 1].replace(",", "").as_str() {
            "blue" => bag.blue = cube_count.parse().unwrap(),
            "green" => bag.green = cube_count.parse().unwrap(),
            "red" => bag.red = cube_count.parse().unwrap(),
            _ => eprintln!("Found a non-rgb cube"),
        }
    }

    return bag;
}