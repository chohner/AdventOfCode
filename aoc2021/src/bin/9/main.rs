use std::collections::HashSet;

// 9
use aoc2021::utils;
type PuzzleInput = Vec<Vec<u32>>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

struct GridPoint {
    pos: Point,
    val: usize,
}

fn is_lowpoint(input: &PuzzleInput, idx: &Point) -> bool {
    let n_right = match idx.x >= input[0].len() - 1 {
        true => u32::MAX,
        false => input[idx.y][idx.x + 1],
    };

    let n_left = match idx.x == 0 {
        true => u32::MAX,
        false => input[idx.y][idx.x - 1],
    };

    let n_up = match idx.y == 0 {
        true => u32::MAX,
        false => input[idx.y - 1][idx.x],
    };

    let n_down = match idx.y >= input.len() - 1 {
        true => u32::MAX,
        false => input[idx.y + 1][idx.x],
    };

    input[idx.y][idx.x] < n_up
        && input[idx.y][idx.x] < n_down
        && input[idx.y][idx.x] < n_left
        && input[idx.y][idx.x] < n_right
}

fn part_1(input: &PuzzleInput) -> Option<i64> {
    let mut lowpoints: Vec<u32> = vec![];
    for (y, row) in input.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if is_lowpoint(input, &Point { x, y }) {
                lowpoints.push(val);
            }
        }
    }
    Some((lowpoints.iter().sum::<u32>() + lowpoints.len() as u32) as i64)
}

fn get_candidates(grid: &PuzzleInput, pos: &Point) -> Vec<Point> {
    let mut candidates = vec![];
    if pos.x > 0 {
        candidates.push(Point {
            x: pos.x - 1,
            y: pos.y,
        })
    }
    if pos.x < grid[0].len() - 1 {
        candidates.push(Point {
            x: pos.x + 1,
            y: pos.y,
        })
    }
    if pos.y > 0 {
        candidates.push(Point {
            x: pos.x,
            y: pos.y - 1,
        })
    }
    if pos.x < grid.len() - 1 {
        candidates.push(Point {
            x: pos.x,
            y: pos.y + 1,
        })
    }
    candidates
}

fn part_2(input: &PuzzleInput) -> Option<i64> {
    let mut lowpoints: Vec<Point> = vec![];
    for (y, row) in input.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let cur_idx = Point { x, y };
            if is_lowpoint(input, &cur_idx) {
                lowpoints.push(cur_idx);
            }
        }
    }

    println!("{:?}", lowpoints);
    for cur_lowpoint in lowpoints {
        let mut current_basin: HashSet<Point> = vec![cur_lowpoint.clone()].into_iter().collect();
        let mut visited: HashSet<Point> = vec![cur_lowpoint.clone()].into_iter().collect();

        let cur_pos = cur_lowpoint;
        let mut candidate_coords = get_candidates(input, &cur_pos);

        for candidate_coord in candidate_coords {
            visited.insert(candidate_coord.clone());
            let candidate_val = input[candidate_coord.y][candidate_coord.x];
            if candidate_val != 9 {
                current_basin.insert(candidate_coord);
            }
            let mut new_candidates = get_candidates(input, &candidate_coord);
            candidate_coords.append(&mut new_candidates);
        }
        println!("current_basin: {:?}", current_basin);
    }

    None
}

fn main() {
    let lines_str = utils::read_file_into_lines(module_path!(), "test.txt").unwrap();
    let puzzle_input: PuzzleInput = lines_str
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    utils::measure_and_print(1, part_1, &puzzle_input);
    utils::measure_and_print(2, part_2, &puzzle_input);
}
