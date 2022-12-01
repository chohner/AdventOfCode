// 10
use anyhow::bail;
use aoc2021::utils;

type PuzzleInput = Vec<Vec<Bracket>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BracketDirection {
    Opening,
    Closing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BracketType {
    Round,
    Square,
    Curly,
    Angle,
}

impl BracketType {
    fn grade(&self) -> usize {
        match self {
            BracketType::Round => 3,
            BracketType::Square => 57,
            BracketType::Curly => 1197,
            BracketType::Angle => 25137,
        }
    }
    fn grade_mult(&self) -> usize {
        match self {
            BracketType::Round => 1,
            BracketType::Square => 2,
            BracketType::Curly => 3,
            BracketType::Angle => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bracket {
    direction: BracketDirection,
    style: BracketType,
}
impl Bracket {
    fn from_char(c: &char) -> anyhow::Result<Bracket> {
        match c {
            '(' => Ok(Bracket {
                style: BracketType::Round,
                direction: BracketDirection::Opening,
            }),
            ')' => Ok(Bracket {
                style: BracketType::Round,
                direction: BracketDirection::Closing,
            }),
            '[' => Ok(Bracket {
                style: BracketType::Square,
                direction: BracketDirection::Opening,
            }),
            ']' => Ok(Bracket {
                style: BracketType::Square,
                direction: BracketDirection::Closing,
            }),
            '{' => Ok(Bracket {
                style: BracketType::Curly,
                direction: BracketDirection::Opening,
            }),
            '}' => Ok(Bracket {
                style: BracketType::Curly,
                direction: BracketDirection::Closing,
            }),
            '<' => Ok(Bracket {
                style: BracketType::Angle,
                direction: BracketDirection::Opening,
            }),
            '>' => Ok(Bracket {
                style: BracketType::Angle,
                direction: BracketDirection::Closing,
            }),
            _ => bail!("unknown char: {}", c),
        }
    }
}

fn part_1(input: &PuzzleInput) -> Option<i64> {
    let mut score = 0;
    for line in input.iter() {
        let mut open_brackets: Vec<BracketType> = vec![];
        for bracket in line {
            if bracket.direction == BracketDirection::Opening {
                open_brackets.push(bracket.style);
            } else {
                if open_brackets.pop().unwrap() != bracket.style {
                    score += bracket.style.grade();
                }
            }
        }
    }
    Some(score as i64)
}

fn part_2(input: &PuzzleInput) -> Option<i64> {
    let mut open_brackets: Vec<Vec<BracketType>> = vec![];

    for line in input.iter() {
        let mut open_brackets_line: Vec<BracketType> = vec![];
        let mut line_is_corrupted = false;
        for bracket in line {
            if bracket.direction == BracketDirection::Opening {
                open_brackets_line.push(bracket.style);
            } else {
                if open_brackets_line.pop().unwrap() != bracket.style {
                    line_is_corrupted = true;
                    break;
                }
            }
        }
        if !line_is_corrupted {
            open_brackets.push(open_brackets_line);
        }
    }

    let mut scores: Vec<usize> = open_brackets
        .iter()
        .map(|btype_line| {
            btype_line
                .iter()
                .rev()
                .fold(0_usize, |acc, btype| acc * 5 + btype.grade_mult())
        })
        .collect();
    scores.sort_unstable();
    Some(scores[(scores.len() - 1) / 2] as i64)
}

fn main() {
    let lines_str = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let puzzle_input: PuzzleInput = lines_str
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| Bracket::from_char(&c).unwrap())
                .collect()
        })
        .collect();
    utils::measure_and_print(1, part_1, &puzzle_input);
    utils::measure_and_print(2, part_2, &puzzle_input);
}
