use aoc2022::utils;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Range {
    min: u32,
    max: u32,
}
impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = s.split_once('-').unwrap();
        let min_fromstr = min.parse::<u32>()?;
        let max_fromstr = max.parse::<u32>()?;
        Ok(Range {
            min: min_fromstr,
            max: max_fromstr,
        })
    }
}

impl Range {
    fn is_overlapping(&self, other: &Range) -> bool {
        (self.min >= other.min && self.max <= other.max)
            || (other.min >= self.min && other.max <= self.max)
    }
}

fn part_1(input: &Vec<Vec<Range>>) -> Option<i64> {
    let count = input
        .iter()
        .filter(|&ranges| ranges[0].is_overlapping(&ranges[1]))
        .count();
    Some(count as i64)
}

fn part_2(input: &Vec<Vec<Range>>) -> Option<i64> {
    None
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<Range>> {
    lines
        .iter()
        .map(|line| {
            line.split(',')
                .map(|range_str| Range::from_str(range_str).unwrap())
                .collect()
        })
        .collect()
}

fn main() {
    let lines = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let inp_split = parse_input(lines);
    utils::measure_and_print(1, part_1, &inp_split);
    utils::measure_and_print(2, part_2, &inp_split);
}
