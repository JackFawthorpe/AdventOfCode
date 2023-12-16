pub fn solve(input: String) -> String {
    first_star(input)
}

// Solves both problems
// The problem can be represented as a quadratic function
// 0 = tx - x^2 - g, where t is the time to beat and g is the distance of the race
// The two solutions are the first integer above the lower solution and 1 under the top solution
fn first_star(input: String) -> String {
    let mut lines = input.lines();
    let times: Vec<f64> = lines.nth(0).unwrap().split_ascii_whitespace().flat_map(|num| num.parse::<f64>()).collect();
    let distances: Vec<f64> = lines.nth(0).unwrap().split_ascii_whitespace().flat_map(|num| num.parse::<f64>()).collect();

    let mut product = 1_f64;

    for index in 0..times.len() {
        let t = times[index];
        let g = distances[index];
        let mut high = t/ 2_f64 + (t * t / 4_f64 - g).sqrt();
        let mut low = t/ 2_f64 - (t * t / 4_f64 - g).sqrt();
        if (high as u64) as f64 == high {high -= 1_f64}
        if (low as u64) as f64 == low {low += 1_f64}
        high = high.floor();
        low = low.ceil();
        product *= high - low + 1_f64
    }
    product.to_string()
}