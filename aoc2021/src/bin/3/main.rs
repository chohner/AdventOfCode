use std::usize;

// 2
use aoc2021::utils;

fn part_1(input: &Vec<String>) -> Option<i64> {
    let mut bit_counter = vec![0; input[0].len()];
    for num_str in input {
        for (idx, char) in num_str.chars().enumerate() {
            if char == '1' {
                bit_counter[idx] += 1
            }
        }
    }
    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    for (power, count) in bit_counter.iter().rev().enumerate() {
        let num: usize = 2usize.pow(power as u32);
        if 2 * count > input.len() {
            gamma += num;
        } else {
            epsilon += num;
        }
    }
    Some((gamma * epsilon) as i64)
}

fn bool_vec_to_num(inp: &Vec<bool>) -> usize {
    inp.iter().rev().enumerate().fold(0, |acc, (pow, bit)| {
        acc + *bit as usize * 2usize.pow(pow as u32)
    })
}

fn find_rating_bin<'a>(
    input: &'a Vec<Vec<bool>>,
    elim_cond: &dyn Fn(usize, usize) -> bool,
) -> &'a Vec<bool> {
    let mut valid_rows: Vec<usize> = (0..input.len()).collect();

    for idx_digit in 0..input[0].len() {
        let ones_counter: usize = valid_rows
            .iter()
            .fold(0, |acc, &idx_row| acc + input[idx_row][idx_digit] as usize);

        let deletion_bit = !elim_cond(ones_counter, valid_rows.len());
        valid_rows = valid_rows
            .into_iter()
            .filter(|&row_idx| input[row_idx][idx_digit] != deletion_bit)
            .to_owned()
            .collect();

        if valid_rows.len() == 1 {
            return &input[valid_rows[0]];
        }
    }
    panic!("OHOH");
}

fn oxygen_comp(counter: usize, valid_row_count: usize) -> bool {
    counter * 2 >= valid_row_count
}
fn scrubber_comp(counter: usize, valid_row_count: usize) -> bool {
    counter * 2 < valid_row_count
}

fn part_2(input: &Vec<String>) -> Option<i64> {
    let input_bin: Vec<Vec<bool>> = input
        .iter()
        .map(|num_str| {
            num_str
                .chars()
                .map(|c| match c {
                    '1' => true,
                    _ => false,
                })
                .collect()
        })
        .collect();

    let oxygen_rating_vec = find_rating_bin(&input_bin, &oxygen_comp);
    let scrubber_rating_vec = find_rating_bin(&input_bin, &scrubber_comp);

    let oxygen_rating = bool_vec_to_num(oxygen_rating_vec);
    let scrubber_rating = bool_vec_to_num(scrubber_rating_vec);
    Some((scrubber_rating * oxygen_rating) as i64)
}

fn main() {
    let lines = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    utils::measure_and_print(1, part_1, &lines);
    utils::measure_and_print(2, part_2, &lines);
}
