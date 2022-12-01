// 13
use aoc2021::utils;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct PuzzleInput {
    points: HashSet<Point>,
    instructions: Vec<FoldInstruction>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
}
impl Point {
    fn from_line(point_line: &str) -> Option<Self> {
        let nums: Vec<&str> = point_line.split(',').collect();
        if nums.len() != 2 {
            return None;
        }
        Some(Point {
            x: u32::from_str(nums[0]).unwrap(),
            y: u32::from_str(nums[1]).unwrap(),
        })
    }
}

#[derive(Debug)]
enum FoldDirection {
    X,
    Y,
}
impl FoldDirection {
    fn from_char(inst_str: &char) -> Result<Self, std::io::Error> {
        match inst_str {
            'x' => Ok(FoldDirection::X),
            'y' => Ok(FoldDirection::Y),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("cannot convert {} to FoldDirection", inst_str),
            )),
        }
    }
}

#[derive(Debug)]
struct FoldInstruction {
    direction: FoldDirection,
    location: u32,
}
impl FoldInstruction {
    fn from_line(inst_line: &str) -> Option<Self> {
        let line_split: Vec<&str> = inst_line.split('=').collect();
        if line_split.len() != 2 {
            return None;
        }
        Some(FoldInstruction {
            direction: FoldDirection::from_char(&line_split[0].chars().last().unwrap()).unwrap(),
            location: u32::from_str(line_split[1]).unwrap(),
        })
    }
}

fn get_max_xy(points: &HashSet<Point>) -> Point {
    let mut max_xy = Point { x: 0, y: 0 };
    for point in points.iter() {
        max_xy.x = max_xy.x.max(point.x);
        max_xy.y = max_xy.y.max(point.y);
    }
    max_xy
}

fn fold(points: &HashSet<Point>, instruction: &FoldInstruction) -> HashSet<Point> {
    let mut new_points = HashSet::new();
    for point in points {
        let new_point = match instruction.direction {
            FoldDirection::X => Point {
                x: if point.x < instruction.location {
                    point.x
                } else {
                    2 * instruction.location - point.x
                },
                y: point.y,
            },
            FoldDirection::Y => Point {
                x: point.x,
                y: if point.y < instruction.location {
                    point.y
                } else {
                    2 * instruction.location - point.y
                },
            },
        };
        new_points.insert(new_point);
    }
    new_points
}

fn part_1(input: &PuzzleInput) -> Option<i64> {
    let mut points = input.points.clone();
    for fold_instruction in input.instructions[0..1].iter() {
        points = fold(&points, fold_instruction);
    }
    Some(points.len() as i64)
}

fn part_2(input: &PuzzleInput) -> Option<i64> {
    let mut points = input.points.clone();
    for fold_instruction in input.instructions.iter() {
        points = fold(&points, fold_instruction);
    }
    let dimensions = get_max_xy(&points);

    let mut out_str = "".to_owned();
    for y in 0..dimensions.y {
        for x in 0..dimensions.x {
            out_str.push(if points.contains(&Point { x, y }) {
                '.'
            } else {
                '#'
            });
        }
        out_str.push('\n');
    }
    println!("Part 2 answer:\n{}", out_str); //CAFJHZCK
    None
}

fn main() {
    let lines_str = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let points_vec: Vec<Point> = lines_str
        .iter()
        .flat_map(|line| Point::from_line(line))
        .collect();

    let mut points: HashSet<Point> = HashSet::new();
    for point in points_vec.iter() {
        points.insert(*point);
    }

    let instructions = lines_str
        .iter()
        .skip(points.len() + 1)
        .map(|inst_str| FoldInstruction::from_line(inst_str).unwrap())
        .collect();

    let puzzle_input = PuzzleInput {
        points,
        instructions,
    };
    utils::measure_and_print(1, part_1, &puzzle_input);
    utils::measure_and_print(2, part_2, &puzzle_input);
}
