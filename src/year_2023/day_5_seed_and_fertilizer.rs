struct Seed {
    current_value: i64,
    current_level: i64,
}


#[derive(Copy, Clone, PartialEq)]
enum SeedState {
    Ready,
    Processed,
    ForNext,
}

#[derive(Copy, Clone)]
struct SeedRange {
    low: i64,
    high: i64,
    processed: SeedState
}

#[derive(Copy, Clone)]
struct Mapping {
    from: i64,
    to: i64,
    count: i64,
}

pub fn solve(input: String) -> String {
    second_star(input)
}

fn second_star(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut seed_ranges: Vec<SeedRange> = get_seed_ranges(lines[0].to_string());
    let mappings: Vec<Vec<Mapping>> = get_mappings(lines);
    process_seed_ranges(&mut seed_ranges, mappings).to_string()
}

// Create the ranges of seeds
fn get_seed_ranges(seed_line: String) -> Vec<SeedRange> {
    let mut seed_ranges = Vec::new();

    let nums: Vec<i64> = seed_line.split_ascii_whitespace().flat_map(|number| number.parse::<i64>()).collect();

    for (index, number) in nums.iter().step_by(2).enumerate() {
        seed_ranges.push(SeedRange {low: *number, high: *number + nums[2 * index + 1] - 1, processed: SeedState::Ready})
    }

    seed_ranges
}

// Process each of the mappings
fn process_seed_ranges(seed_ranges: &mut Vec<SeedRange>, mappings: Vec<Vec<Mapping>>) -> i64 {
    
    for layer_index in 0..mappings.len() {
        for mapping in mappings[layer_index].iter() {
            
            let mut seed_index: usize = 0;

            while seed_index < seed_ranges.len() {
                let seed_range = seed_ranges[seed_index];

                // If the Seed is out of the range of the current map or its already gone through this layer
                if seed_range.low >= mapping.from + mapping.count
                || seed_range.high < mapping.from
                || seed_range.processed == SeedState::Processed 
                || seed_range.processed == SeedState::ForNext {seed_index += 1; continue;}

                // Break the range into the mapped and unmapped sections
                let new_ranges : Vec<SeedRange> = split_seed_range(seed_range, *mapping);
                new_ranges.iter().for_each(|new_seed_range| seed_ranges.push(*new_seed_range));
                seed_ranges[seed_index].processed = SeedState::Processed;

                seed_index += 1
            }
            
        }

        // Remove all the processed seeds as they are represented as subranges now
        seed_ranges.retain(|seed_range| !(seed_range.processed == SeedState::Processed));

        // Reset the seeds for the next layer
        seed_ranges.iter_mut().for_each(|range| range.processed = SeedState::Ready);
    }

    seed_ranges.iter().map(|seed_range| seed_range.low).min().unwrap()
}


// Takes a seed range affected by a map and returns the sub ranges
fn split_seed_range(seed_range: SeedRange, mapping: Mapping) -> Vec<SeedRange> {
    let mut new_ranges: Vec<SeedRange> = Vec::new();

    // The section below the mapping that is unchanged
    if seed_range.low < mapping.from {
        new_ranges.push(SeedRange{low: seed_range.low, high: mapping.from - 1, processed: SeedState::Ready});
    }

    // The section above the mapping that is unchanged
    if seed_range.high >= mapping.from + mapping.count {
        new_ranges.push(SeedRange{low: mapping.from + mapping.count, high: seed_range.high, processed: SeedState::Ready});
    }

    // The altered section
    new_ranges.push(SeedRange {
        low: seed_range.low.max(mapping.from) + (mapping.to - mapping.from), 
        high: seed_range.high.min(mapping.from + mapping.count - 1) + (mapping.to - mapping.from), 
        processed: SeedState::ForNext});

    new_ranges
}

#[allow(dead_code)]
fn first_star(input: String) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut seeds: Vec<Seed> = get_seeds(lines[0].to_string());
    let mappings: Vec<Vec<Mapping>> = get_mappings(lines);
    let final_positions = process_seeds(&mut seeds, mappings);

    final_positions.iter().min().unwrap().to_string()
}

// Takes each seed through each of the layers
fn process_seeds(seeds: &mut Vec<Seed>, mappings: Vec<Vec<Mapping>>) -> Vec<i64> {

    for (layer_index, layer) in mappings.iter().enumerate() {
        for mapping in layer {
            for seed in seeds.iter_mut() {

                // If the seed doesnt fit the map or is already changed
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

// Parse the mappings
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

// Get a mapping from a single line
fn get_line_mapping(line: String) -> Mapping {
    let numbers: Vec<i64> = line.split_ascii_whitespace().flat_map(|num| num.parse::<i64>()).collect();
    Mapping {from: numbers[1], to: numbers[0], count: numbers[2]}
}

// Get the seeds from the seed line
fn get_seeds(seed_line: String) -> Vec<Seed> {
    seed_line
    .split_ascii_whitespace()
    .flat_map(|number| number.parse::<i64>())
    .map(|number| Seed {current_value: number, current_level: 0})
    .collect()
}