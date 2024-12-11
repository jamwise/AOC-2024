use aoc_2024::{is_in_bounds, log_output, parse_string};
use std::collections::HashSet;

struct Walk {
    directions: [(isize, isize); 4],
    path_endings: HashSet<(usize, usize)>,
    distinct_paths: usize,
}

impl Walk {
    fn new(from: (usize, usize), grid: &Vec<Vec<u8>>) -> Self {
        let mut walk_struct = Self {
            directions: [(0, -1), (1, 0), (0, 1), (-1, 0)],
            path_endings: HashSet::new(),
            distinct_paths: 0,
        };
        walk_struct.walk(from, grid, 0);

        walk_struct
    }

    fn walk(&mut self, from: (usize, usize), grid: &Vec<Vec<u8>>, number: u8) -> () {
        if number == 9 {
            self.distinct_paths += 1;
            self.path_endings.insert(from);
        }

        for (x, y) in self.directions {
            let next_position = (from.0.wrapping_add_signed(x), from.1.wrapping_add_signed(y));
            if !is_in_bounds(&next_position, grid) {
                continue;
            }
            let next_number = grid[next_position.1][next_position.0];
            if next_number == number + 1 {
                self.walk(next_position, grid, next_number);
            }
        }
    }
}

pub fn grid_iter<F>(puzzle: &str, mut function: F) -> ()
where
    F: FnMut(usize, usize, &Vec<Vec<u8>>) -> (),
{
    let grid = parse_string(puzzle, vec![]);

    let grid: Vec<Vec<u8>> = grid
        .iter()
        .map(|row| row.iter().map(|cell| cell.parse().unwrap()).collect())
        .collect();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                function(x, y, &grid);
            }
        }
    }
}

fn part1(puzzle: &str) -> i64 {
    let mut paths = 0;

    grid_iter(puzzle, |x, y, grid| {
        paths += Walk::new((x, y), &grid).path_endings.len();
    });

    paths as i64
}

fn part2(puzzle: &str) -> i64 {
    let mut paths = 0;

    grid_iter(puzzle, |x, y, grid| {
        paths += Walk::new((x, y), &grid).distinct_paths;
    });

    paths as i64
}

fn main() {
    log_output(1, || part1(include_str!("data.txt")));
    log_output(2, || part2(include_str!("data.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("test.txt")), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test.txt")), 81);
    }
}
