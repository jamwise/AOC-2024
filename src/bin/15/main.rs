use aoc_2024::{log_output, parse_string};

fn find_robot(map: &Vec<Vec<char>>) -> (usize, usize) {
    map.iter()
        .enumerate()
        .find_map(|(row, vec)| vec.iter().position(|&c| c == '@').map(|col| (row, col)))
        .unwrap()
}

fn move_1d(vec: Vec<char>, direction: isize) -> (bool, Vec<char>) {
    if let Some(pos) = vec.iter().position(|&x| x == '@') {
        if vec[(pos as isize + direction) as usize] == '#' {
            return (false, vec);
        }

        let mut look_forward = direction;
        while vec[(pos as isize + look_forward) as usize] != '#' {
            if vec[(pos as isize + look_forward) as usize] == '.' {
                let mut new_vec = vec.clone();

                new_vec.remove((pos as isize + look_forward) as usize);
                new_vec.insert(pos as usize, '.');

                return (true, new_vec);
            }
            look_forward += direction;
        }

        (false, vec)
    } else {
        (false, vec)
    }
}

fn move_2d(map: &Vec<Vec<char>>, direction: isize) -> (bool, Vec<Vec<char>>) {
    let (row_number, column_number) = find_robot(&map);
    let next_row = (row_number as isize + direction) as usize;
    let next_char = map[next_row][column_number];
    if next_char == '#' {
        return (false, map.clone());
    }
    if next_char != '[' && next_char != ']' {
        let mut new_map: Vec<Vec<char>> = map.clone();
        new_map[row_number][column_number] = '.';
        new_map[next_row][column_number] = '@';
        return (true, new_map);
    }

    let mut boxes: Vec<((usize, usize), (usize, usize))> = Vec::new();

    if next_char == '[' {
        boxes.push(((column_number, next_row), (column_number + 1, next_row)));
    } else if next_char == ']' {
        boxes.push(((column_number - 1, next_row), (column_number, next_row)));
    }

    let range: Vec<usize> = if direction == -1 {
        (0..next_row + 1).rev().collect()
    } else {
        (next_row..map.len() - 1).collect()
    };

    for row in range {
        for i in 0..boxes.len() {
            let ((x1, y1), (x2, y2)) = boxes[i];
            if y1 != row {
                continue;
            }

            let next_left_pos = (x1, (y1 as isize + direction) as usize);
            let next_right_pos = (x2, (y2 as isize + direction) as usize);

            let next_left = map[next_left_pos.1][next_left_pos.0];
            let next_right = map[next_right_pos.1][next_right_pos.0];

            if next_left == '#' || next_right == '#' {
                return (false, map.clone());
            }

            if next_left == '[' {
                boxes.push((next_left_pos, next_right_pos));
            } else if next_left == ']' {
                boxes.push(((x1 - 1, (y1 as isize + direction) as usize), next_left_pos));
            }
            if next_right == '[' {
                boxes.push((next_right_pos, (x2 + 1, (y2 as isize + direction) as usize)));
            }
        }
    }

    boxes.reverse();

    let mut new_map: Vec<Vec<char>> = map.clone();

    for ((x1, y1), (x2, y2)) in boxes {
        new_map[y1][x1] = '.';
        new_map[y2][x2] = '.';
        new_map[(y1 as isize + direction) as usize][x1] = '[';
        new_map[(y2 as isize + direction) as usize][x2] = ']';
    }
    new_map[row_number][column_number] = '.';
    new_map[next_row][column_number] = '@';

    (true, new_map)
}

fn move_robot(map: Vec<Vec<char>>, direction: char, with_2d: bool) -> Vec<Vec<char>> {
    let mut map = map.clone();
    match direction {
        '^' | 'v' => {
            let direction_num = if direction == '^' { -1 } else { 1 };
            let (_, column_number) = find_robot(&map);

            if with_2d {
                let (moved, new_map) = move_2d(&map, direction_num);
                if !moved {
                    return map;
                }
                return new_map;
            } else {
                let column: Vec<char> = map.iter().map(|row| row[column_number]).collect();
                let (moved, new_column) = move_1d(column, direction_num);
                if !moved {
                    return map;
                }

                for (row, new_value) in map.iter_mut().zip(new_column.iter()) {
                    row[column_number] = *new_value;
                }
            }

            map
        }
        '<' | '>' => {
            let direction_num = if direction == '<' { -1 } else { 1 };
            let (row_number, _) = find_robot(&map);
            let row = map[row_number].clone();
            let (moved, new_row) = move_1d(row, direction_num);
            if !moved {
                return map;
            }
            map[row_number] = new_row;

            map
        }
        _ => {
            panic!("Invalid direction");
        }
    }
}

fn follow_movements(map: &str, movements: &str, with_2d: bool) -> usize {
    let movements: Vec<Vec<char>> = parse_string(movements, vec![]);
    let movements: Vec<char> = movements.into_iter().flatten().collect();
    let mut map: Vec<Vec<char>> = parse_string(map, vec![]);

    let mut gps_total = 0;

    for movement in movements {
        map = move_robot(map, movement, with_2d);
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'O' || map[y][x] == '[' {
                gps_total += 100 * y + x;
            }
        }
    }

    gps_total
}

fn main() {
    log_output(1, || {
        follow_movements(
            include_str!("map.txt"),
            include_str!("movements.txt"),
            false,
        )
    });
    log_output(2, || {
        follow_movements(
            include_str!("map_2.txt"),
            include_str!("movements.txt"),
            true,
        )
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            follow_movements(
                include_str!("test_map.txt"),
                include_str!("test_movements.txt"),
                false,
            ),
            10092
        );
        assert_eq!(
            move_1d("##@.O..#".chars().collect(), -1),
            (false, "##@.O..#".chars().collect())
        );
        assert_eq!(
            move_1d("##@.O..#".chars().collect(), 1),
            (true, "##.@O..#".chars().collect())
        );
        assert_eq!(
            move_1d("##.@O..#".chars().collect(), 1),
            (true, "##..@O.#".chars().collect())
        );
        assert_eq!(
            move_1d("##.@OO.#".chars().collect(), 1),
            (true, "##..@OO#".chars().collect())
        );
        assert_eq!(
            move_1d("#.#@.O.#".chars().collect(), -1),
            (false, "#.#@.O.#".chars().collect())
        );
        assert_eq!(
            move_1d("#.#..@O#".chars().collect(), 1),
            (false, "#.#..@O#".chars().collect())
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            follow_movements(
                include_str!("test_map_2.txt"),
                include_str!("test_movements.txt"),
                true,
            ),
            9021
        );
    }
}
