use aoc2022::utils;

fn part_1(input: &Vec<usize>) -> Option<i64> {
    Some(input.iter().max().unwrap().clone() as i64)
}
fn part_2(input: &Vec<usize>) -> Option<i64> {
    let mut input_sorted: Vec<usize> = input.to_owned();
    input_sorted.sort();
    Some((input_sorted.iter().rev().take(3).sum::<usize>()) as i64)
}

fn parse_input(lines: &Vec<String>) -> Vec<usize> {
    let mut split_sums: Vec<usize> = vec![];
    let mut accum = 0;

    for line in lines.iter() {
        match line.parse::<usize>() {
            Ok(num) => accum += num,
            Err(_) => {
                split_sums.push(accum);
                accum = 0;
            }
        }
    }
    split_sums
}

fn main() {
    let lines = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let split_sums = parse_input(&lines);
    utils::measure_and_print(1, part_1, &split_sums);
    utils::measure_and_print(2, part_2, &split_sums);
}
