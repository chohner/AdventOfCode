// 6
use aoc2021::utils;

fn simulate_day(fishes: &mut Vec<usize>) {
    let count = fishes.len();
    for idx in 0..count {
        if fishes[idx] == 0 {
            fishes[idx] = 6;
            fishes.push(8);
        } else {
            fishes[idx] -= 1;
        }
    }
}

fn simulate_days(fishes: &mut Vec<usize>, days: usize) {
    for _ in 0..days {
        simulate_day(fishes);
    }
}

fn part_1(input: &Vec<usize>) -> Option<i64> {
    let mut fishes = input.clone();
    simulate_days(&mut fishes, 80);
    Some(fishes.len() as i64)
}

fn part_2(_input: &Vec<usize>) -> Option<i64> {
    let start_idx: u32 = 6;
    let steps_to_double = 7;
    let _exponent = f64::exp(f64::ln(2.) / steps_to_double as f64);

    let mut fishes = vec![start_idx as usize];
    let mut len_counter = 0;
    let mut prev_len = 1;
    for idx in 0..80 {
        let len = fishes.len();
        if len == prev_len {
            len_counter += 1;
        } else {
            len_counter = 1;
            prev_len = len;
        }

        let start_offset = steps_to_double - start_idx - 1;
        let exp_growth = 2_usize.pow((idx + start_offset) / 7);
        println!(
            "step {}: len {} for {} steps, exp: {}, diff: {}",
            idx,
            prev_len,
            len_counter,
            exp_growth,
            exp_growth as i64 - prev_len as i64,
        );
        simulate_day(&mut fishes);
    }
    // println!("exponent: {}", exponent);
    // simulate_days(&mut fishes, 8);
    Some(fishes.len() as i64)
}

fn main() {
    let lines = utils::read_file_into_lines(module_path!(), "test.txt").unwrap();
    let numbers = utils::delim_line_into_usize(&lines[0], ',');
    utils::measure_and_print(1, part_1, &numbers);
    utils::measure_and_print(2, part_2, &numbers);
}
