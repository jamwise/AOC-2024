use std::collections::HashSet;

use aoc_2024::{log_output, parse_string};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
    floor_size: (isize, isize),
}

impl Robot {
    fn new(px: isize, py: isize, vx: isize, vy: isize, floor_size: (isize, isize)) -> Self {
        Self {
            position: (px, py),
            velocity: (vx, vy),
            floor_size,
        }
    }
    fn seconds_forward(&mut self, seconds: isize) {
        let new_x = self.velocity.0 * seconds + self.position.0;
        let new_y = self.velocity.1 * seconds + self.position.1;

        self.position.0 =
            (((new_x % self.floor_size.0) + self.floor_size.0) % self.floor_size.0).abs();
        self.position.1 =
            (((new_y % self.floor_size.1) + self.floor_size.1) % self.floor_size.1).abs();
    }
}

fn solve_puzzle(puzzle: &str, floor_size: (isize, isize)) -> isize {
    let puzzle: Vec<Vec<isize>> = parse_string(puzzle, vec!['p', 'v', '=', ',', ' ']);
    let bx = floor_size.0 / 2;
    let by = floor_size.1 / 2;

    let mut quadrants = [0; 4];

    for robot_data in puzzle {
        let [px, py, vx, vy] = robot_data.try_into().unwrap();
        let mut robot = Robot::new(px, py, vx, vy, floor_size);
        robot.seconds_forward(100);
        match robot.position {
            (x, y) if x < bx && y < by => quadrants[0] += 1,
            (x, y) if x > bx && y < by => quadrants[1] += 1,
            (x, y) if x < bx && y > by => quadrants[2] += 1,
            (x, y) if x > bx && y > by => quadrants[3] += 1,
            _ => (),
        }
    }

    quadrants.iter().product()
}

fn print_floor(robots: &HashSet<(isize, isize)>, floor_size: (isize, isize)) {
    for row in 0..floor_size.1 {
        for column in 0..floor_size.0 {
            if robots.contains(&(column, row)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("\n\n");
}

fn find_tree(puzzle: &str, floor_size: (isize, isize)) {
    let puzzle: Vec<Vec<isize>> = parse_string(puzzle, vec!['p', 'v', '=', ',', ' ']);
    let mut robot_list = Vec::new();

    for robot_data in puzzle {
        let [px, py, vx, vy] = robot_data.try_into().unwrap();
        robot_list.push(Robot::new(px, py, vx, vy, floor_size));
    }

    let mut seconds = 0;
    loop {
        seconds += 1;
        for robot in robot_list.iter_mut() {
            robot.seconds_forward(1);
        }

        let robots: HashSet<(isize, isize)> =
            robot_list.iter().map(|robot| robot.position).collect();

        let mut tree_top = false;

        for row in 0..floor_size.1 {
            for column in 0..floor_size.0 {
                if robots.contains(&(column, row))
                    && robots.contains(&(column - 1, row + 1))
                    && robots.contains(&(column, row + 1))
                    && robots.contains(&(column + 1, row + 1))
                    && robots.contains(&(column - 2, row + 2))
                    && robots.contains(&(column - 1, row + 2))
                    && robots.contains(&(column, row + 2))
                    && robots.contains(&(column + 1, row + 2))
                    && robots.contains(&(column + 2, row + 2))
                {
                    tree_top = true;
                }
            }
        }

        if tree_top {
            println!("Seconds: {}", seconds);
            print_floor(&robots, floor_size);
            break;
        }
    }
}

fn main() {
    log_output(1, || solve_puzzle(include_str!("data.txt"), (101, 103)));
    find_tree(include_str!("data.txt"), (101, 103));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_puzzle(include_str!("test.txt"), (11, 7)), 12);
    }
}
