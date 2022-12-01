// 17
use aoc2021::utils;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct Probe {
    velocity: Point,
    position: Point,
}
impl Probe {
    fn new(velocity: Point) -> Probe {
        Probe {
            position: Point { x: 0, y: 0 },
            velocity,
        }
    }

    fn step(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.velocity.y -= 1;
        if self.velocity.x > 0 {
            self.velocity.x -= 1;
        }
    }

    fn inside_area(&self, target_area: &TargetArea) -> bool {
        self.position.x >= target_area.min.x
            && self.position.x <= target_area.max.x
            && self.position.y >= target_area.min.y
            && self.position.y <= target_area.max.y
    }
}

fn simulate_y_max(velocity: Point, target_area: &TargetArea) -> Option<i32> {
    let mut probe = Probe::new(velocity.to_owned());
    let mut y_max = 0;
    while probe.position.y > target_area.min.y {
        if probe.position.y > y_max {
            y_max = probe.position.y;
        }
        probe.step();

        if probe.inside_area(target_area) {
            return Some(y_max);
        }
    }
    None
}

struct TargetArea {
    min: Point,
    max: Point,
}

fn minimum_vel_x(target_area: &TargetArea) -> i32 {
    let mut dist_x = 0;
    let mut vel_x_min = 0;

    while dist_x < target_area.min.x {
        vel_x_min += 1;
        dist_x += vel_x_min;
    }
    vel_x_min
}

fn part_1(target_area: &TargetArea) -> Option<i64> {
    // x is independet - find smallest one
    let vel_x = minimum_vel_x(target_area);

    // On its way down, the probe will pass 0 with inverted starting velocity.
    // Therefore, highest ossible vel_y is falling from 0 -> target_area.min.y
    let y_max = simulate_y_max(
        Point {
            x: vel_x,
            y: -target_area.min.y - 1,
        },
        target_area,
    );

    Some(y_max.unwrap() as i64)
}

fn part_2(target_area: &TargetArea) -> Option<i64> {
    let vel_x_min = minimum_vel_x(target_area);
    let vel_y_max = -target_area.min.y - 1;
    let vel_y_min = target_area.min.y;
    let vel_x_max = target_area.max.x;

    let mut velocities_valid: Vec<Point> = vec![];
    for vel_y in vel_y_min..vel_y_max + 1 {
        for vel_x in vel_x_min..vel_x_max + 1 {
            if simulate_y_max(Point { x: vel_x, y: vel_y }, target_area).is_some() {
                velocities_valid.push(Point { x: vel_x, y: vel_y });
            }
        }
    }

    Some(velocities_valid.len() as i64)
}

fn main() {
    let _target_area_test = TargetArea {
        min: Point { x: 20, y: -10 },
        max: Point { x: 30, y: -5 },
    };
    let target_area_puzzle = TargetArea {
        min: Point { x: 85, y: -163 },
        max: Point { x: 145, y: -108 },
    };
    utils::measure_and_print(1, part_1, &target_area_puzzle);
    utils::measure_and_print(2, part_2, &target_area_puzzle);
}
