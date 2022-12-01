// 20
use aoc2021::utils;
use core::fmt;

// lets play around with the Newtype pattern
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types

#[derive(Debug, Clone, Copy)]
struct Pixel(bool);

impl Pixel {
    fn from_char(c: &char) -> Option<Self> {
        match c {
            '#' => Some(Pixel(true)),
            '.' => Some(Pixel(false)),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self.0 {
            true => '#',
            false => '.',
        }
    }

    fn on(&self) -> bool {
        self.0
    }
}

#[derive(Debug)]
struct Enhancement(Vec<Pixel>);
impl fmt::Display for Enhancement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut outstr = "".to_owned();
        for val in self.0.iter() {
            outstr.push(val.to_char());
        }
        write!(f, "{}", outstr)
    }
}

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<Pixel>>);
impl Grid {
    fn get_i32(&self, row: i32, col: i32) -> Option<Pixel> {
        if row >= 0 && col >= 0 && (row as usize) < self.0.len() && (col as usize) < self.0.len() {
            return Some(self.0[row as usize][col as usize].clone());
        }
        None
    }

    fn count_on(&self) -> usize {
        self.0
            .iter()
            .flatten()
            .fold(0, |acc, pix| acc + pix.on() as usize)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut outstr = "".to_owned();
        for row in self.0.iter() {
            for val in row.iter() {
                outstr.push(val.to_char());
            }
            outstr.push('\n');
        }
        write!(f, "{}", outstr)
    }
}

#[derive(Debug)]
struct PuzzleInput {
    enhancement: Enhancement,
    image: Grid,
}

struct Kernel([[Pixel; 3]; 3]);
impl Kernel {
    fn as_num(&self) -> usize {
        self.0
            .iter()
            .flatten()
            .rev()
            .enumerate()
            .fold(0, |acc, (idx, pix)| {
                acc + pix.0 as usize * 2_usize.pow(idx as u32)
            })
    }

    fn enhance(&self, enhancement: &Enhancement) -> Pixel {
        enhancement.0[self.as_num()].clone()
    }
}

fn enhance(image: &Grid, enhancement: &Enhancement, default_pixel: Pixel) -> Grid {
    let mut new_image: Vec<Vec<Pixel>> = vec![];

    for row in 0..image.0.len() + 2 {
        new_image.push(vec![]);
        for col in 0..image.0[0].len() + 2 {
            let old_row = row as i32 - 1;
            let old_col = col as i32 - 1;
            let kernel = Kernel([
                [
                    image
                        .get_i32(old_row - 1, old_col - 1)
                        .unwrap_or(default_pixel),
                    image
                        .get_i32(old_row - 1, old_col - 0)
                        .unwrap_or(default_pixel),
                    image
                        .get_i32(old_row - 1, old_col + 1)
                        .unwrap_or(default_pixel),
                ],
                [
                    image.get_i32(old_row, old_col - 1).unwrap_or(default_pixel),
                    image.get_i32(old_row, old_col - 0).unwrap_or(default_pixel),
                    image.get_i32(old_row, old_col + 1).unwrap_or(default_pixel),
                ],
                [
                    image
                        .get_i32(old_row + 1, old_col - 1)
                        .unwrap_or(default_pixel),
                    image
                        .get_i32(old_row + 1, old_col - 0)
                        .unwrap_or(default_pixel),
                    image
                        .get_i32(old_row + 1, old_col + 1)
                        .unwrap_or(default_pixel),
                ],
            ]);
            kernel.enhance(enhancement);
            new_image[row].push(kernel.enhance(enhancement));
        }
    }
    Grid(new_image)
}

fn part_1(input: &PuzzleInput) -> Option<i64> {
    let mut image = input.image.clone();
    for idx in 0..2 {
        let default_pixel = if idx % 2 == 0 {
            input.enhancement.0.last().unwrap().to_owned()
        } else {
            input.enhancement.0[0]
        };
        image = enhance(&image, &input.enhancement, default_pixel);
    }
    Some(image.count_on() as i64)
}

fn part_2(input: &PuzzleInput) -> Option<i64> {
    let mut image = input.image.clone();
    for idx in 0..50 {
        let default_pixel = if idx % 2 == 0 {
            input.enhancement.0.last().unwrap().to_owned()
        } else {
            input.enhancement.0[0]
        };
        image = enhance(&image, &input.enhancement, default_pixel);
    }
    Some(image.count_on() as i64)
}

fn main() {
    let lines_str = utils::read_file_into_lines(module_path!(), "input.txt").unwrap();
    let enhancement = Enhancement(
        lines_str[0]
            .chars()
            .map(|c| Pixel::from_char(&c).unwrap())
            .collect(),
    );

    let image = Grid(
        lines_str
            .iter()
            .skip(2)
            .map(|line| {
                line.chars()
                    .map(|c| Pixel::from_char(&c).unwrap())
                    .collect::<Vec<Pixel>>()
            })
            .collect(),
    );

    let puzzle_input = PuzzleInput { enhancement, image };
    utils::measure_and_print(1, part_1, &puzzle_input);
    utils::measure_and_print(2, part_2, &puzzle_input);
}
