use std::collections::HashSet;

use aoc2022::utils;

fn char_intersection(chars1: &[char], chars2: &[char]) -> Vec<char> {
    let bag1: HashSet<char> = chars1.iter().cloned().collect();
    let bag2: HashSet<char> = chars2.iter().cloned().collect();
    bag1.intersection(&bag2).cloned().collect()
}

fn split_and_intersect(chars: &Vec<char>) -> Vec<char> {
    let (part1, part2) = chars.split_at(chars.len() / 2);
    char_intersection(part1, part2)
}

fn part_1(input: &Vec<Vec<char>>) -> Option<i64> {
    let prio_sum = input
        .iter()
        .map(|chars| split_and_intersect(chars))
        .map(|chars| char_to_prio(chars[0]))
        .sum::<u32>();

    Some(prio_sum as i64)
}

fn part_2(input: &Vec<Vec<char>>) -> Option<i64> {
    let intersection_sum: u32 = input
        .chunks(3)
        .map(|bags| char_intersection(&char_intersection(&bags[0], &bags[1]), &bags[2]))
        .map(|intersec| char_to_prio(intersec[0]))
        .sum();

    Some(intersection_sum as i64)
}

fn char_to_prio(c: char) -> u32 {
    let unicode_offset = if c.is_lowercase() { 96 } else { 38 };
    c as u32 - unicode_offset
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn main() {
    let lines = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let lines_prio = parse_input(&lines);
    utils::measure_and_print(1, part_1, &lines_prio);
    utils::measure_and_print(2, part_2, &lines_prio);
}
