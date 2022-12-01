// 14
use aoc2021::utils;
use std::collections::HashMap;

type InsertionRules = HashMap<char, HashMap<char, char>>;

#[derive(Debug)]
struct PuzzleInput {
    template: String,
    insertion_rules: InsertionRules,
}

fn run_insertion(template: &Vec<char>, insertion_rules: &InsertionRules) -> Vec<char> {
    let mut out: Vec<char> = vec![];
    out.reserve(2 * template.len() - 1);
    for idx in 0..template.len() - 1 {
        out.push(template[idx]);
        let insert = insertion_rules
            .get(&template[idx])
            .unwrap()
            .get(&template[idx + 1])
            .unwrap()
            .clone();
        out.push(insert);
    }
    out.push(template[template.len() - 1]);
    out
}

fn count_occurences(template: &Vec<char>) -> HashMap<char, usize> {
    let mut out = HashMap::new();
    for char in template {
        if !out.contains_key(char) {
            out.insert(char.clone(), 0_usize);
        }
        *out.get_mut(char).unwrap() += 1;
    }
    out
}

struct MinMax {
    min: usize,
    max: usize,
}

fn count_min_max(template: &Vec<char>) -> MinMax {
    let occurences = count_occurences(&template);
    let mut min = usize::MAX;
    let mut max = usize::MIN;
    for &val in occurences.values() {
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }
    MinMax { min, max }
}

fn part_1(input: &PuzzleInput) -> Option<i64> {
    let mut template: Vec<char> = input.template.chars().collect();
    for _ in 0..10 {
        template = run_insertion(&template, &input.insertion_rules);
    }
    let minmax = count_min_max(&template);
    Some((minmax.max - minmax.min) as i64)
}

fn part_2(input: &PuzzleInput) -> Option<i64> {
    let mut template: Vec<char> = input.template.chars().collect();
    for idx in 0..40 {
        template = run_insertion(&template, &input.insertion_rules);
        if idx > 10 {
            let minmax = count_min_max(&template);

            println!("idx {} - min: {}, max: {}", idx, minmax.min, minmax.max);
        }
    }
    // Some((minmax.max - minmax.min) as i64)
    None
}

fn build_insertion_rules(lines: &[String]) -> InsertionRules {
    let mut insertion_rules = HashMap::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        if !insertion_rules.contains_key(&chars[0]) {
            insertion_rules.insert(chars[0], HashMap::new());
        }
        insertion_rules
            .get_mut(&chars[0])
            .unwrap()
            .insert(chars[1], chars[6]);
    }
    insertion_rules
}

fn main() {
    let lines_str = utils::read_file_into_lines(module_path!(), "test.txt").unwrap();
    let puzzle_input = PuzzleInput {
        template: lines_str[0].to_owned(),
        insertion_rules: build_insertion_rules(&lines_str[2..]),
    };
    utils::measure_and_print(1, part_1, &puzzle_input);
    utils::measure_and_print(2, part_2, &puzzle_input);
}
