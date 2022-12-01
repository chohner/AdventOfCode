// 11
use aoc2021::utils;
use std::fmt;

type PuzzleInput = Vec<Vec<Octopus>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Octopus {
    energy: u8,
    flash: bool,
}
impl Octopus {
    fn from_char(c: &char) -> Octopus {
        Octopus {
            energy: c.to_digit(10).unwrap() as u8,
            flash: false,
        }
    }

    fn inc_if_not_zero(&mut self) {
        if self.energy > 0 {
            self.energy += 1;
        }
    }
}

impl fmt::Display for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.energy)
    }
}

fn grid_string(inp: &PuzzleInput) -> String {
    let mut s = String::new();
    for row in inp.iter() {
        row.iter().for_each(|oct| s += &oct.to_string());
        s += "\n";
    }
    s
}

fn simulate_step(octopusses: &mut PuzzleInput) -> usize {
    octopusses
        .iter_mut()
        .for_each(|line| line.iter_mut().for_each(|oct| oct.energy += 1));

    let mut flashes_this_step = 0;
    let mut step_finished = false;
    while !step_finished {
        step_finished = true;
        for idx_row in 0..octopusses.len() {
            for idx_col in 0..octopusses[0].len() {
                if octopusses[idx_row][idx_col].energy > 9 {
                    octopusses[idx_row][idx_col].energy = 0;
                    octopusses[idx_row][idx_col].flash = true;
                    flashes_this_step += 1;
                    step_finished = false;
                    let row_before = idx_row > 0;
                    let row_after = idx_row < octopusses.len() - 1;
                    let col_before = idx_col > 0;
                    let col_after = idx_col < octopusses[0].len() - 1;
                    if row_after {
                        octopusses[idx_row + 1][idx_col].inc_if_not_zero();
                    }
                    if row_before {
                        octopusses[idx_row - 1][idx_col].inc_if_not_zero()
                    }
                    if col_before {
                        octopusses[idx_row][idx_col - 1].inc_if_not_zero();
                        if row_before {
                            octopusses[idx_row - 1][idx_col - 1].inc_if_not_zero();
                        }
                        if row_after {
                            octopusses[idx_row + 1][idx_col - 1].inc_if_not_zero();
                        }
                    }
                    if col_after {
                        octopusses[idx_row][idx_col + 1].inc_if_not_zero();
                        if row_before {
                            octopusses[idx_row - 1][idx_col + 1].inc_if_not_zero();
                        }
                        if row_after {
                            octopusses[idx_row + 1][idx_col + 1].inc_if_not_zero();
                        }
                    }
                }
            }
        }
    }

    flashes_this_step
}

fn part_1(input: &PuzzleInput) -> Option<i64> {
    let mut octopusses = input.clone();
    let step_count = 100;

    let total_flashes = (0..step_count)
        .into_iter()
        .fold(0, |acc, _| acc + simulate_step(&mut octopusses));

    Some(total_flashes as i64)
}

fn part_2(input: &PuzzleInput) -> Option<i64> {
    let mut octopusses = input.clone();
    let octopus_count = octopusses.len() * octopusses[0].len();
    let mut step_count = 1;
    while simulate_step(&mut octopusses) != octopus_count {
        step_count += 1;
    }
    Some(step_count as i64)
}

fn main() {
    let lines_str = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let puzzle_input: PuzzleInput = lines_str
        .iter()
        .map(|line| line.chars().map(|c| Octopus::from_char(&c)).collect())
        .collect();
    utils::measure_and_print(1, part_1, &puzzle_input);
    utils::measure_and_print(2, part_2, &puzzle_input);
}
