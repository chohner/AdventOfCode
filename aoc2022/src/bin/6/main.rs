use aoc2022::utils;

fn part_1(input: &Vec<String>) -> Option<i64> {
    let char_count = 3;
    for line in input.iter() {
        let mut last_idx = 0;
        let mut last_elements = Vec::from_iter(line.chars().take(char_count));

        for (idx, char) in line.chars().skip(char_count).enumerate() {
            let mut last_elements_dedup = last_elements.clone();
            last_elements_dedup.sort();
            last_elements_dedup.dedup();

            if !last_elements.contains(&char) && last_elements_dedup.len() == char_count {
                return Some((idx + char_count + 1) as i64);
            }

            last_elements[last_idx] = char;
            last_idx = (last_idx + 1) % char_count;
        }
    }
    None
}

fn part_2(input: &Vec<String>) -> Option<i64> {
    let char_count = 13;
    for line in input.iter() {
        let mut last_idx = 0;
        let mut last_elements = Vec::from_iter(line.chars().take(char_count));

        for (idx, char) in line.chars().skip(char_count).enumerate() {
            let mut last_elements_dedup = last_elements.clone();
            last_elements_dedup.sort();
            last_elements_dedup.dedup();

            if !last_elements.contains(&char) && last_elements_dedup.len() == char_count {
                return Some((idx + char_count + 1) as i64);
            }

            last_elements[last_idx] = char;
            last_idx = (last_idx + 1) % char_count;
        }
    }
    None
}

fn main() {
    let lines = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    utils::measure_and_print(1, part_1, &lines);
    utils::measure_and_print(2, part_2, &lines);
}
