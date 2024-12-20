use std::collections::HashMap;

use aoc_2024::{log_output, parse_string};

type Point = (usize, usize);
type Direction = (isize, isize);

const DIRECTIONS: [Direction; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Clone)]
struct Item {
    steps: usize,
    x: usize,
    y: usize,
}

struct Track {
    hash_map: HashMap<Point, usize>,
    array: Vec<Item>,
}

impl Track {
    fn new() -> Self {
        Self {
            hash_map: HashMap::new(),
            array: vec![],
        }
    }

    fn insert(&mut self, point: Point, item: Item) {
        self.hash_map.insert(point, self.array.len());
        self.array.push(item);
    }

    fn get(&self, point: &Point) -> Option<&Item> {
        self.hash_map.get(point).map(|&index| &self.array[index])
    }
}

fn get_track(puzzle: &str) -> Track {
    let mut track: Track = Track::new();
    let map: Vec<Vec<char>> = parse_string(puzzle, vec![]);

    let mut dir: Direction = (0, 0);
    let mut step = 0;

    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    let mut x: Option<usize> = None;
    let mut y: Option<usize> = None;

    let height = map.len();
    let width = map[0].len();

    for row in 0..height {
        for col in 0..width {
            match map[row][col] {
                'S' => {
                    start = Some((col, row));
                    track.insert(
                        (col, row),
                        Item {
                            steps: step,
                            x: col,
                            y: row,
                        },
                    );
                    step += 1;
                }
                'E' => end = Some((col, row)),
                _ => (),
            }
        }
    }
    x = Some(start.unwrap().0);
    y = Some(start.unwrap().1);

    loop {
        'dirs: for (dir_x, dir_y) in &DIRECTIONS {
            let new_x = (x.unwrap() as isize + dir_x) as usize;
            let new_y = (y.unwrap() as isize + dir_y) as usize;

            if map[new_y][new_x] == '#' {
                continue 'dirs;
            }
            if dir.0 as isize * dir_x + dir.1 as isize * dir_y == -1 {
                continue 'dirs;
            }
            x = Some(new_x);
            y = Some(new_y);
            dir = (*dir_x, *dir_y);
            track.insert(
                (new_x, new_y),
                Item {
                    steps: step,
                    x: new_x,
                    y: new_y,
                },
            );
            step += 1;
        }
        if end.unwrap().0 == x.unwrap() && end.unwrap().1 == y.unwrap() {
            break;
        }
    }

    track
}

fn part1(puzzle: &str) -> usize {
    let track = get_track(puzzle);
    let mut x: Option<usize> = None;
    let mut y: Option<usize> = None;
    let mut answer = 0;

    for item in &track.array {
        x = Some(item.x);
        y = Some(item.y);
        let cheat_from = item.steps as isize;

        for (dir_x, dir_y) in &DIRECTIONS {
            let dir_x = *dir_x * 2;
            let dir_y = *dir_y * 2;

            let new_x = (x.unwrap() as isize + dir_x) as usize;
            let new_y = (y.unwrap() as isize + dir_y) as usize;

            let cheat_to = track.get(&(new_x, new_y));
            if cheat_to.is_some() && cheat_to.unwrap().steps as isize - cheat_from >= 100 + 2 {
                answer += 1;
            }
        }
    }

    answer
}

fn part2(puzzle: &str) -> usize {
    let track = get_track(puzzle);
    let mut answer = 0;

    for i in 0..&track.array.len() - (100 + 2) {
        let from = &track.array[i];
        let x = from.x as isize;
        let y = from.y as isize;
        let cheat_from = from.steps as isize;

        for j in (i + 1)..track.array.len() {
            let to = &track.array[j];
            let x2 = to.x as isize;
            let y2 = to.y as isize;
            let cheat_to = to.steps as isize;
            let x_diff = (y - y2).abs();
            let y_diff = (x - x2).abs();
            let distance = x_diff + y_diff;
            if distance <= 20 && cheat_to - cheat_from >= 100 + distance {
                answer += 1;
            }
        }
    }

    answer
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
        assert_eq!(part1(include_str!("test.txt")), 44);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test.txt")), 0);
    }
}
