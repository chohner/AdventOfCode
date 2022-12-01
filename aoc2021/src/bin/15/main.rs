// 15
use aoc2021::utils;
use std::collections::{HashMap, VecDeque};

type NodeGrid = Vec2D<Node>;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}
impl Position {
    fn manhatten_dist(&self, dest: &Position) -> u32 {
        ((dest.x as i64 - self.x as i64).abs() + (dest.y as i64 - self.y as i64).abs()) as u32
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy, PartialOrd, Ord)]
struct Node {
    cost: u32,
    pos: Position,
}
impl Node {
    fn manhatten_dist(&self, dest: &Node) -> u32 {
        self.pos.manhatten_dist(&dest.pos)
    }
}

#[derive(Debug, Clone)]
struct Vec2D<Node> {
    n_rows: usize,
    n_cols: usize,
    data: Vec<Node>,
}
impl Vec2D<Node> {
    fn get_pos(&self, pos: &Position) -> Node {
        // Is position is outside of grid: wrap costs 1 .. 9
        let idx = (pos.y % self.n_rows) * self.n_cols + (pos.x % self.n_cols);
        let row_overflow = (pos.y / self.n_rows) as u32;
        let col_overflow = (pos.x / self.n_cols) as u32;
        Node {
            pos: pos.clone(),
            cost: (self.data[idx].cost + row_overflow + col_overflow - 1) % 9 + 1,
        }
    }
}

fn reconstruct_path(came_from: &HashMap<Node, Node>, current: &Node) -> Vec<Node> {
    let mut cur_node = current.to_owned();
    let mut total_path = vec![cur_node];
    while came_from.contains_key(&cur_node) {
        cur_node = came_from.get(&cur_node).unwrap().to_owned();
        total_path.push(cur_node);
    }
    total_path.reverse();
    total_path
}

fn get_neighbours(pos: &Position, limit: &Position) -> Vec<Position> {
    let mut neighbours = vec![];
    if pos.x > 0 {
        neighbours.push(Position {
            x: pos.x - 1,
            y: pos.y,
        })
    }
    if pos.x < limit.x {
        neighbours.push(Position {
            x: pos.x + 1,
            y: pos.y,
        });
    }
    if pos.y > 0 {
        neighbours.push(Position {
            x: pos.x,
            y: pos.y - 1,
        })
    }
    if pos.y < limit.y {
        neighbours.push(Position {
            x: pos.x,
            y: pos.y + 1,
        });
    }
    neighbours
}

fn a_star_search(start: &Node, goal: &Node, grid: &NodeGrid) -> Vec<Node> {
    // https://en.wikipedia.org/wiki/A*_search_algorithm
    // let start = grid.get_pos(start_pos);
    // let goal = grid.get_pos(goal_pos);

    let mut open_set: VecDeque<Node> = VecDeque::from([start.to_owned()]);
    let mut g_score: HashMap<Node, u32> = HashMap::with_capacity(1);
    g_score.insert(start.clone(), 0);
    let mut f_score: HashMap<Node, u32> = HashMap::with_capacity(1);
    f_score.insert(start.clone(), start.manhatten_dist(&goal));
    let mut came_from: HashMap<Node, Node> = HashMap::new();

    while !open_set.is_empty() {
        let current_node = open_set.pop_front().unwrap();

        if current_node == *goal {
            return reconstruct_path(&came_from, &current_node);
        }

        for neighbour_pos in get_neighbours(&current_node.pos, &goal.pos).iter() {
            let neighbour_node = grid.get_pos(neighbour_pos);
            let tentative_g_score = g_score.get(&current_node).unwrap() + neighbour_node.cost;

            if !g_score.contains_key(&neighbour_node)
                || g_score.get(&neighbour_node).unwrap() > &tentative_g_score
            {
                came_from.insert(neighbour_node, current_node);
                g_score.insert(neighbour_node, tentative_g_score);
                let cur_f_score = tentative_g_score + neighbour_node.manhatten_dist(&goal);
                f_score.insert(neighbour_node, cur_f_score);

                // insert node into open_set, sorted by f_score
                match open_set
                    .binary_search_by(|&elem| f_score.get(&elem).unwrap().cmp(&cur_f_score))
                {
                    Ok(pos) => open_set.insert(pos, neighbour_node),
                    Err(pos) => open_set.insert(pos, neighbour_node),
                }
            }
        }
    }
    vec![]
}

fn part_1(input: &NodeGrid) -> Option<i64> {
    let start = input.get_pos(&Position { x: 0, y: 0 });
    let goal = input.get_pos(&Position {
        y: input.n_rows - 1,
        x: input.n_cols - 1,
    });
    let optimal_path = a_star_search(&start, &goal, &input);
    let cost = optimal_path.iter().fold(0, |acc, x| acc + x.cost) - start.cost;
    Some(cost as i64)
}

fn part_2(input: &NodeGrid) -> Option<i64> {
    let start = input.get_pos(&Position { x: 0, y: 0 });
    let goal = input.get_pos(&Position {
        y: 5 * input.n_rows - 1,
        x: 5 * input.n_cols - 1,
    });
    let optimal_path = a_star_search(&start, &goal, &input);
    let cost = optimal_path.iter().fold(0, |acc, x| acc + x.cost) - start.cost;

    Some(cost as i64)
}

fn main() {
    let lines_str = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let node_grid_flat: Vec<Node> = lines_str
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, digit_char)| Node {
                pos: Position { x, y },
                cost: digit_char.to_digit(10).unwrap() as u32,
            })
        })
        .collect();

    let puzzle_input = Vec2D {
        n_rows: lines_str.len(),
        n_cols: lines_str[0].len(),
        data: node_grid_flat,
    };

    utils::measure_and_print(1, part_1, &puzzle_input);
    utils::measure_and_print(2, part_2, &puzzle_input);
}
