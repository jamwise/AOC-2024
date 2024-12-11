use std::collections::HashSet;
use aoc_2024::{parse_string, log_output};

fn get_disk_map(puzzle: &Vec<usize>) -> Vec<i64> {
    let mut disk_map: Vec<i64> = Vec::new();

    for (id, chunk) in puzzle.chunks(2).enumerate() {
        let id = id as i64;
        match chunk {
            [file_size, space_size] => {
                disk_map.extend(vec![id; *file_size]);
                disk_map.extend(vec![-1; *space_size])
            }
            [file_size] => disk_map.extend(vec![id; *file_size]),
            _ => {
                panic!("Error at index: {}", id);
            }
        }
    }

    disk_map
}

fn part1(puzzle: &str) -> i64 {
    let puzzle: Vec<Vec<usize>> = parse_string(puzzle, vec![]);
    let puzzle = &puzzle[0];

    let disk_map = get_disk_map(&puzzle);

    let mut checksum = 0;

    let mut reverse_index = disk_map.len();

    for forward_index in 0..disk_map.len() {
        if forward_index >= reverse_index {
            break;
        };
        let id = disk_map[forward_index];
        if id == -1 {
            reverse_index -= 1;
            while disk_map[reverse_index] == -1 {
                reverse_index -= 1;
            }
            let reverse_id = disk_map[reverse_index];
            checksum += reverse_id * forward_index as i64;
        } else {
            checksum += id * forward_index as i64;
        }
    }

    checksum
}

fn part2(puzzle: &str) -> i64 {
    let puzzle: Vec<Vec<usize>> = parse_string(puzzle, vec![]);
    let puzzle = &puzzle[0];

    let mut disk_map: Vec<i64> = Vec::new();

    let mut moved_ids = HashSet::new();

    for (id, chunk) in puzzle.chunks(2).enumerate() {
        let mut reverse_index = if puzzle.len() % 2 == 0 {
            puzzle.len() - 2
        } else {
            puzzle.len() - 1
        };

        match chunk {
            [file_size, space_size] => {
                let mut space_size = space_size.clone();

                disk_map.extend(vec![
                    if moved_ids.contains(&id) {
                        -1
                    } else {
                        id as i64
                    };
                    *file_size
                ]);

                while reverse_index > id * 2 {
                    let reverse_file_id = reverse_index / 2;
                    let reverse_file_size = puzzle[reverse_index];

                    if reverse_file_size <= space_size && !moved_ids.contains(&reverse_file_id) {
                        disk_map.extend(vec![reverse_file_id as i64; reverse_file_size]);
                        space_size -= reverse_file_size;
                        moved_ids.insert(reverse_file_id);
                    }
                    reverse_index -= 2;
                    if space_size == 0 {
                        break;
                    }
                }
                disk_map.extend(vec![-1; space_size]);
            }
            [file_size] => disk_map.extend(vec![
                if moved_ids.contains(&id) {
                    -1
                } else {
                    id as i64
                };
                *file_size
            ]),
            _ => {
                panic!("Error at index: {}", id);
            }
        }
    }

    disk_map.iter().enumerate().fold(0, |acc, (index, value)| {
        if *value == -1 {
            return acc;
        }
        acc + (index as i64 * value)
    })
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
        assert_eq!(part1(include_str!("test.txt")), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test.txt")), 2858);
    }
}
