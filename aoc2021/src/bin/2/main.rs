// 2
use anyhow;
use aoc2021::utils;
use std::str::FromStr;

enum SubDirection {
    Forward,
    Up,
    Down,
}
impl FromStr for SubDirection {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "forward" => Ok(SubDirection::Forward),
            "up" => Ok(SubDirection::Up),
            "down" => Ok(SubDirection::Down),
            _ => anyhow::bail!("pattern does not contain `{}`"),
        }
    }
}

struct SubInstruction {
    direction: SubDirection,
    count: usize,
}
impl SubInstruction {
    fn from_line(s: &str) -> anyhow::Result<Self> {
        let line_split: Vec<&str> = s.split(' ').collect();
        let dir_fromstr = line_split[0].parse::<SubDirection>()?;
        let count_fromstr = line_split[1].parse::<usize>()?;

        Ok(SubInstruction {
            direction: dir_fromstr,
            count: count_fromstr,
        })
    }
}

fn part_1(input: &Vec<SubInstruction>) -> Option<i64> {
    let mut depth = 0;
    let mut pos_h = 0;
    for instruction in input {
        match instruction.direction {
            SubDirection::Down => depth += instruction.count,
            SubDirection::Forward => pos_h += instruction.count,
            SubDirection::Up => depth -= instruction.count,
        }
    }
    Some((depth * pos_h) as i64)
}

fn part_2(input: &Vec<SubInstruction>) -> Option<i64> {
    let mut depth = 0;
    let mut pos_h = 0;
    let mut aim = 0;
    for instruction in input {
        match instruction.direction {
            SubDirection::Down => aim += instruction.count,
            SubDirection::Up => aim -= instruction.count,
            SubDirection::Forward => {
                pos_h += instruction.count;
                depth += aim * instruction.count
            }
        }
    }
    Some((depth * pos_h) as i64)
}

fn main() {
    let lines = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let instructions: Vec<SubInstruction> = lines
        .iter()
        .map(|line| SubInstruction::from_line(line).unwrap())
        .collect();
    utils::measure_and_print(1, part_1, &instructions);
    utils::measure_and_print(2, part_2, &instructions);
}
