// 5
use anyhow;
use aoc2021::utils;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        let s_split: Vec<&str> = s.split(',').collect();
        Ok(Point {
            x: s_split[0].parse::<i32>()?,
            y: s_split[1].parse::<i32>()?,
        })
    }
}

struct Line {
    p0: Point,
    p1: Point,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.p0, self.p1)
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(line: &str) -> anyhow::Result<Self> {
        let line_split: Vec<&str> = line.split(" -> ").collect();
        Ok(Line {
            p0: line_split[0].parse::<Point>()?,
            p1: line_split[1].parse::<Point>()?,
        })
    }
}
impl Line {
    fn is_horizontal(&self) -> bool {
        self.p0.y == self.p1.y
    }

    fn is_vertical(&self) -> bool {
        self.p0.x == self.p1.x
    }

    fn get_points(&self) -> Vec<Point> {
        let mut direction = Point {
            x: self.p1.x - self.p0.x,
            y: self.p1.y - self.p0.y,
        };
        let mag = direction.x.abs().max(direction.y.abs());
        direction.x /= mag;
        direction.y /= mag;

        (0..=mag)
            .map(|idx| Point {
                x: self.p0.x + idx * direction.x,
                y: self.p0.y + idx * direction.y,
            })
            .collect()
    }
}

type Field = HashMap<Point, usize>;

fn visit_point(point: Point, field: &mut Field) {
    let counter = field.entry(point).or_insert(0);
    *counter += 1;
}

fn part_1(input: &Vec<Line>) -> Option<i64> {
    let mut hit_count: Field = HashMap::new();
    for line in input {
        if line.is_horizontal() || line.is_vertical() {
            for p in line.get_points() {
                visit_point(p, &mut hit_count);
            }
        }
    }
    Some(
        hit_count
            .iter()
            .fold(0, |acc: i64, val| acc + (val.1 >= &2) as i64),
    )
}
fn part_2(input: &Vec<Line>) -> Option<i64> {
    let mut hit_count: Field = HashMap::new();
    for line in input {
        for p in line.get_points() {
            visit_point(p, &mut hit_count);
        }
    }
    Some(
        hit_count
            .iter()
            .fold(0, |acc: i64, val| acc + (val.1 >= &2) as i64),
    )
}

fn main() {
    let lines_str = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let lines = lines_str
        .iter()
        .map(|l| l.parse::<Line>().unwrap())
        .collect();
    utils::measure_and_print(1, part_1, &lines);
    utils::measure_and_print(2, part_2, &lines);
}
