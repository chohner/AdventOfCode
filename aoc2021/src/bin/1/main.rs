// 1
use aoc2021::utils;

fn part_1(input: &Vec<usize>) -> Option<i64> {
    let mut inc_counter = 0;
    for (idx, number) in input.iter().enumerate() {
        if idx >= 1 && number > &input[idx - 1] {
            inc_counter += 1;
        }
    }
    Some(inc_counter)
}

fn part_2(input: &Vec<usize>) -> Option<i64> {
    let mut inc_counter = 0;
    for (idx, number) in input.iter().enumerate() {
        if idx >= 1 && idx < input.len() - 2 {
            let current_win = number + input[idx + 1] + input[idx + 2];
            let last_win = input[idx - 1] + number + input[idx + 1];
            if current_win > last_win {
                inc_counter += 1;
            }
        }
    }
    Some(inc_counter)
}

fn main() {
    let lines = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let numbers = utils::lines_to_usize_vec(&lines);
    utils::measure_and_print(1, part_1, &numbers);
    utils::measure_and_print(2, part_2, &numbers);
}
