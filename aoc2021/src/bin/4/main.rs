// 4
use aoc2021::utils;

const ROWCOUNT: usize = 5;
const COLCOUNT: usize = 5;

#[derive(Debug, Clone, Copy)]
struct Field {
    num: usize,
    selected: bool,
}

type Board = Vec<Vec<Field>>;

struct Game {
    instructions: Vec<usize>,
    boards: Vec<Board>,
}

fn line_into_usize(s: &str, delim: char) -> Vec<usize> {
    let line_split: Vec<&str> = s.split(delim).collect();
    line_split
        .iter()
        .filter_map(|&c| c.parse::<usize>().ok())
        .collect()
}

fn lines_into_board(lines: &[String]) -> Board {
    lines[0..5]
        .iter()
        .map(|line| {
            line_into_usize(line, ' ')
                .iter()
                .map(|&num| Field {
                    num,
                    selected: false,
                })
                .collect()
        })
        .collect()
}

fn check_row(board: &Board, row_idx: usize) -> bool {
    board[row_idx]
        .iter()
        .fold(true, |acc, field| acc & field.selected)
}

fn check_col(board: &Board, col_idx: usize) -> bool {
    board
        .iter()
        .fold(true, |acc, row| acc & row[col_idx].selected)
}

fn sum_unselected_fields(board: &Board) -> usize {
    board.iter().fold(0, |total_acc, row| {
        row.iter().fold(total_acc, |acc, field| {
            acc + !field.selected as usize * field.num
        })
    })
}

fn part_1(input: &Game) -> Option<i64> {
    let mut boards = input.boards.clone();
    for new_num in input.instructions.clone() {
        for board in boards.iter_mut() {
            'rowloop: for row_idx in 0..ROWCOUNT {
                for col_idx in 0..COLCOUNT {
                    if board[row_idx][col_idx].num == new_num {
                        board[row_idx][col_idx].selected = true;

                        if check_row(board, row_idx) | check_col(board, col_idx) {
                            return Some((sum_unselected_fields(board) * new_num) as i64);
                        }
                        break 'rowloop;
                    }
                }
            }
        }
    }
    None
}

fn part_2(input: &Game) -> Option<i64> {
    let mut boards = input.boards.clone();
    let board_count = boards.len();
    let mut board_won = vec![false; board_count];
    let mut boards_won_count = 0;
    for new_num in input.instructions.clone() {
        for (board_idx, board) in boards.iter_mut().enumerate() {
            if !board_won[board_idx] {
                'rowloop: for row_idx in 0..ROWCOUNT {
                    for col_idx in 0..COLCOUNT {
                        if board[row_idx][col_idx].num == new_num {
                            board[row_idx][col_idx].selected = true;

                            if check_row(board, row_idx) | check_col(board, col_idx) {
                                board_won[board_idx] = true;
                                boards_won_count += 1;
                                if boards_won_count == board_count {
                                    return Some((sum_unselected_fields(board) * new_num) as i64);
                                }
                            }
                            break 'rowloop;
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let lines = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let boards: Vec<Board> = lines[2..]
        .chunks(6)
        .map(|board_lines| lines_into_board(board_lines))
        .collect();
    let game = Game {
        instructions: line_into_usize(&lines[0], ','),
        boards,
    };

    utils::measure_and_print(1, part_1, &game);
    utils::measure_and_print(2, part_2, &game);
}
