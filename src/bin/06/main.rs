use aoc_2024::{parse_string, log_output};
use std::collections::HashMap;
use std::collections::HashSet;

fn map_floor(floor: &Vec<Vec<String>>) -> HashMap<(i64, i64), String> {
    let mut floor_map = HashMap::new();
    for (y, row) in floor.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            floor_map.insert((x as i64, y as i64), tile.clone());
        }
    }
    floor_map
}

fn position_in_bounds(position: &(i64, i64), floor: &Vec<Vec<String>>) -> bool {
    position.1 >= 0
        && position.1 < floor.len() as i64
        && position.0 >= 0
        && position.0 < floor[0].len() as i64
}

fn walk_path(
    floor: &Vec<Vec<String>>,
    start: (i64, i64),
    direction: usize,
    floor_map: &HashMap<(i64, i64), String>,
    visited_positions: HashSet<(i64, i64)>,
    loop_check: bool,
) -> i64 {
    let directions: Vec<(i64, i64)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut direction = direction.clone();
    let mut visited_positions = visited_positions.clone();
    let mut visited_obstacles: Vec<((i64, i64), (i64, i64))> = Vec::new();
    let mut current_position = start.clone();

    let mut loop_obstacles = HashSet::new();

    while position_in_bounds(&current_position, &floor) {
        let next_position = (
            current_position.0 + directions[direction].0,
            current_position.1 + directions[direction].1,
        );

        if !position_in_bounds(&next_position, &floor) {
            break;
        }

        if floor_map.get(&next_position) != Some(&String::from("#")) {
            if loop_check && !loop_obstacles.contains(&next_position) {
                let mut new_floor_map = floor_map.clone();
                new_floor_map.insert(next_position, String::from("#"));
                if walk_path(
                    floor,
                    current_position.clone(),
                    direction,
                    &new_floor_map,
                    HashSet::new(),
                    false,
                ) == -1
                {
                    loop_obstacles.insert(next_position.clone());
                }
            }

            visited_positions.insert(next_position.clone());
            current_position = next_position;
        } else {
            if !loop_check
                && visited_obstacles.contains(&(current_position, next_position))
            {
                return -1;
            }
            visited_obstacles.push((current_position.clone(), next_position.clone()));
            direction = (direction + 1) % 4;
        }
    }

    if loop_check {
        return loop_obstacles.len() as i64;
    }

    visited_positions.len() as i64
}

fn part1(floor_str: &str) -> i64 {
    let floor = parse_string(floor_str, vec![]);
    let mut visited_positions: HashSet<(i64, i64)> = HashSet::new();
    let floor_map = map_floor(&floor);

    let current_position = floor_map
        .iter()
        .find(|(_, tile)| **tile == String::from("^"))
        .unwrap()
        .0
        .clone();

    visited_positions.insert(current_position.clone());

    walk_path(
        &floor,
        current_position,
        0,
        &floor_map,
        visited_positions,
        false,
    )
}

fn part2(floor_str: &str) -> i64 {
    let floor = parse_string(floor_str, vec![]);
    let mut visited_positions: HashSet<(i64, i64)> = HashSet::new();
    let floor_map = map_floor(&floor);

    let current_position = floor_map
        .iter()
        .find(|(_, tile)| **tile == String::from("^"))
        .unwrap()
        .0
        .clone();

    visited_positions.insert(current_position.clone());

    walk_path(
        &floor,
        current_position,
        0,
        &floor_map,
        visited_positions,
        true,
    )
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
        assert_eq!(part1(include_str!("test.txt")), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test.txt")), 6);
    }
}
