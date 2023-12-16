struct Seed {
    current_value: i64,
    current_level: i64,
}

struct Mapping {
    from: i64,
    to: i64,
    count: i64,
}

pub fn solve(input: String) -> String {
    first_star(input)
}

#[allow(dead_code)]
fn second_star(input: String) -> String {
    input
}

fn first_star(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut seeds: Vec<Seed> = get_seeds(lines[0].to_string());
    let mappings: Vec<Vec<Mapping>> = get_mappings(lines);
    let final_positions = process_seeds(&mut seeds, mappings);

    final_positions.iter().min().unwrap().to_string()
}

fn process_seeds(seeds: &mut Vec<Seed>, mappings: Vec<Vec<Mapping>>) -> Vec<i64> {

    for (layer_index, layer) in mappings.iter().enumerate() {
        for mapping in layer {
            for seed in seeds.iter_mut() {
                if seed.current_value < mapping.from 
                || seed.current_value >=mapping.from + mapping.count 
                || seed.current_level == (layer_index + 1) as i64 { continue;}
                *seed = Seed {
                    current_value: seed.current_value + mapping.to - mapping.from,
                    current_level: (layer_index +  1) as i64}
            }
        }
    }

    seeds.iter().map(|seed| seed.current_value).collect()
}

fn get_mappings(lines: Vec<&str>) -> Vec<Vec<Mapping>> {
    let mut mappings: Vec<Vec<Mapping>> = Vec::new();
    let mut current_mapping: Vec<Mapping> = Vec::new();
    for index in 1..lines.len() {
        if lines[index].len() == 0 {continue;}
        if !lines[index].chars().nth(0).unwrap().is_numeric() {
            mappings.push(current_mapping);
            current_mapping = Vec::new();
            continue;
        }
        current_mapping.push(get_line_mapping(lines[index].to_string()));
    }
    mappings.push(current_mapping);
    mappings
}

fn get_line_mapping(line: String) -> Mapping {
    let numbers: Vec<i64> = line.split_ascii_whitespace().flat_map(|num| num.parse::<i64>()).collect();
    Mapping {from: numbers[1], to: numbers[0], count: numbers[2]}
}

fn get_seeds(seed_line: String) -> Vec<Seed> {
    seed_line
    .split_ascii_whitespace()
    .flat_map(|number| number.parse::<i64>())
    .map(|number| Seed {current_value: number, current_level: 0})
    .collect()
}