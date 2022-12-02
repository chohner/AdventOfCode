use aoc2022::utils;
// 14328 too high

#[derive(Debug, PartialEq, Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Hand {
    pub fn from_string(input: &char) -> Hand {
        match input {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissor,
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissor,
            _ => todo!(),
        }
    }

    pub fn play_against(&self, other: &Hand) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }

        if (self == &Hand::Paper && other == &Hand::Rock)
            || (self == &Hand::Rock && other == &Hand::Scissor)
            || (self == &Hand::Scissor && other == &Hand::Paper)
        {
            return Outcome::Win;
        }
        Outcome::Loss
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    pub fn score(&self) -> usize {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug)]
struct Game {
    own: Hand,
    other: Hand,
}

impl Game {
    pub fn score(&self) -> usize {
        self.own as usize + self.own.play_against(&self.other).score()
    }
}

fn part_1(input: &Vec<Game>) -> Option<i64> {
    let score_sum = input.iter().fold(0, |acc, game| acc + game.score());
    Some(score_sum as i64)
}

fn part_2(input: &Vec<Game>) -> Option<i64> {
    let mut total_score = 0;
    for game in input.iter() {
        // 1. Remap own hand to desired outcome
        let desired_outcome = match game.own {
            Hand::Rock => Outcome::Loss,
            Hand::Paper => Outcome::Draw,
            Hand::Scissor => Outcome::Win,
        };

        // 2. Pick own hand based on desired outcome
        let own_hand: Hand = match desired_outcome {
            Outcome::Win => match game.other {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissor,
                Hand::Scissor => Hand::Rock,
            },
            Outcome::Loss => match game.other {
                Hand::Rock => Hand::Scissor,
                Hand::Paper => Hand::Rock,
                Hand::Scissor => Hand::Paper,
            },
            Outcome::Draw => game.other,
        };

        // 3. accumulate total score
        total_score += desired_outcome.score() + own_hand as usize;
    }
    Some(total_score as i64)
}

fn parse_input(lines: &Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];

    for line in lines.iter() {
        let chars: Vec<char> = line.chars().collect();
        games.push(Game {
            own: Hand::from_string(&chars.get(2).unwrap()),
            other: Hand::from_string(&chars.get(0).unwrap()),
        })
    }
    games
}

fn main() {
    let lines = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let games = parse_input(&lines);
    utils::measure_and_print(1, part_1, &games);
    utils::measure_and_print(2, part_2, &games);
}
