// 5
use anyhow;
use aoc2021::utils;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use std::usize;

type Connection = String;
type PuzzleInput = Vec<DigitMapping>;
type Digit = String;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct DigitMapping {
    connections: Vec<Connection>,
    digits: Vec<Digit>,
}
impl FromStr for DigitMapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let s_split: Vec<&str> = s.split(" | ").collect();

        Ok(DigitMapping {
            connections: s_split[0].split(' ').map(|s| s.to_owned()).collect(),
            digits: s_split[1].split(' ').map(|s| s.to_owned()).collect(),
        })
    }
}

fn digit_is_unique(digit: &str) -> bool {
    match digit.chars().count() {
        2 | 3 | 4 | 7 => true, // digit 1, 7, 4 ,8
        _ => false,
    }
}

fn count_simplesegments(mapping: &DigitMapping) -> i64 {
    mapping
        .digits
        .iter()
        .fold(0, |acc, digit| acc + digit_is_unique(digit) as i64)
}

fn part_1(input: &PuzzleInput) -> Option<i64> {
    Some(
        input
            .iter()
            .fold(0, |acc, digitmap| acc + count_simplesegments(digitmap)),
    )
}

fn strs_of_len_in_vec(strings: &Vec<String>, len: usize) -> Vec<&String> {
    strings
        .iter()
        .filter(|&s| s.chars().count() == len)
        .collect()
}

fn not_all_chars_in_target(search_str: &str, target: &str) -> bool {
    search_str.chars().any(|c| !target.contains(c))
}

fn build_decoder(conns: &Vec<String>) -> HashMap<char, char> {
    // 1. find len2 (1)
    let len2 = strs_of_len_in_vec(conns, 2)[0];

    // 2. find len3 (7)
    let len3 = strs_of_len_in_vec(conns, 3)[0];

    // 3. save difference as segment a
    let segment_a = len3.chars().find(|&c| !len2.contains(c)).unwrap();

    // 4. find all len6 (0, 6, 9)
    let len6s = strs_of_len_in_vec(conns, 6);

    // 5. find len6 that has any of len2 (1) missing
    let len6_missing = len6s
        .iter()
        .find(|&&s| not_all_chars_in_target(len2, s))
        .unwrap();

    // 6. save missing char as segment c
    let segment_c = len2.chars().find(|&c| !len6_missing.contains(c)).unwrap();

    // 7. other char in len2 (1) is segment f
    let segment_f = len2.chars().find(|&c| c != segment_c).unwrap();

    // 8. find len4 (4)
    let len4 = strs_of_len_in_vec(conns, 4)[0];

    // 9. from remaining len6, find the one with any char of len4 (4) missing
    let len6_missing_2 = len6s
        .iter()
        .find(|&&s| &s != len6_missing && not_all_chars_in_target(len4, s))
        .unwrap();

    //10: save missing char as segment d
    let segment_d = len4.chars().find(|&c| !len6_missing_2.contains(c)).unwrap();

    //11: save remaining unknown char in len4 (4) as segment b
    let segment_b = len4
        .chars()
        .find(|&c| !(c == segment_d || c == segment_f || c == segment_c))
        .unwrap();

    //12: find last remaining len6 (9)
    let remaining_len6 = len6s
        .iter()
        .find(|&s| s != len6_missing && s != len6_missing_2)
        .unwrap();

    // 13: save unknown char in last len6 as segment g
    let segment_g = remaining_len6
        .chars()
        .find(|&c| {
            !(c == segment_d
                || c == segment_f
                || c == segment_c
                || c == segment_b
                || c == segment_a)
        })
        .unwrap();

    // 14: save remaining char as segment e
    let segment_e = strs_of_len_in_vec(conns, 7)[0]
        .chars()
        .find(|&c| {
            !(c == segment_d
                || c == segment_f
                || c == segment_c
                || c == segment_b
                || c == segment_a
                || c == segment_g)
        })
        .unwrap();

    let mut decoder: HashMap<char, char> = HashMap::with_capacity(8);
    decoder.insert(segment_a, 'a');
    decoder.insert(segment_b, 'b');
    decoder.insert(segment_b, 'b');
    decoder.insert(segment_c, 'c');
    decoder.insert(segment_d, 'd');
    decoder.insert(segment_e, 'e');
    decoder.insert(segment_f, 'f');
    decoder.insert(segment_g, 'g');
    decoder
}

fn decodesegment(encodedsegments: &str, decoder: &HashMap<char, char>) -> usize {
    let mut decoded: Vec<char> = encodedsegments
        .chars()
        .map(|c| decoder.get(&c).unwrap().to_owned())
        .collect();
    decoded.sort_unstable();
    let decoded_str: String = decoded.into_iter().collect();
    match decoded_str.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => 0,
    }
}

fn decode_number(endcoded_num: &Vec<String>, decoder: &HashMap<char, char>) -> usize {
    endcoded_num.iter().enumerate().fold(0, |acc, (idx, str)| {
        acc + 10_usize.pow((endcoded_num.len() - 1 - idx) as u32) * decodesegment(str, decoder)
    })
}

fn decodesegment_mapping(input: &DigitMapping) -> usize {
    let decoder = build_decoder(&input.connections);
    decode_number(&input.digits, &decoder)
}

fn part_2(input: &PuzzleInput) -> Option<i64> {
    Some(
        input
            .iter()
            .map(|inp| decodesegment_mapping(inp))
            .sum::<usize>() as i64,
    )
}

fn main() {
    let lines_str = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let puzzle_input = lines_str
        .iter()
        .map(|l| DigitMapping::from_str(l).unwrap())
        .collect();
    utils::measure_and_print(1, part_1, &puzzle_input);
    utils::measure_and_print(2, part_2, &puzzle_input);
}
