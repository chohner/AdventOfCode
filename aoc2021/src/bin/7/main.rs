// 7
use aoc2021::utils;

fn fuel_costs_lin(positions: &Vec<usize>, dest: i64) -> i64 {
    positions
        .iter()
        .fold(0, |acc, &pos| (acc + (pos as i64 - dest).abs()))
}

fn fuel_costs_grow(positions: &Vec<usize>, dest: i64) -> i64 {
    // growing at 1,3,6,10 ... = n * (n + 1) / 2
    positions.iter().fold(0, |acc, &pos| {
        acc + (pos as i64 - dest).abs() * ((pos as i64 - dest).abs() + 1) / 2
    })
}

fn vec_median(nums: &Vec<usize>) -> f32 {
    let num_count = nums.len();
    let mut nums_sorted = nums.clone();
    nums_sorted.sort_unstable();
    if num_count % 2 == 1 {
        return nums_sorted[(num_count - 1) / 2] as f32;
    } else {
        return (nums_sorted[num_count / 2 - 1] + nums_sorted[num_count / 2]) as f32 / 2.;
    }
}

fn part_1(input: &Vec<usize>) -> Option<i64> {
    let dest = vec_median(input).round() as i64;
    Some(fuel_costs_lin(input, dest))
}

fn part_2(input: &Vec<usize>) -> Option<i64> {
    // brute force :'(
    let mut guess = vec_median(input).round() as i64;
    let mut last_cost = i64::MAX;
    let mut current_cost = fuel_costs_grow(input, guess);
    while current_cost < last_cost {
        last_cost = current_cost;
        guess += 1;
        current_cost = fuel_costs_grow(input, guess);
    }
    Some(last_cost)
}

fn main() {
    let lines = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let numbers = utils::delim_line_into_usize(&lines[0], ',');
    utils::measure_and_print(1, part_1, &numbers);
    utils::measure_and_print(2, part_2, &numbers);
}
