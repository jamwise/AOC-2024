use aoc_2024::parse_string;
use std::collections::HashMap;
use std::collections::HashSet;

fn position_in_bounds(position: &(i64, i64), rows: &Vec<Vec<String>>) -> bool {
    position.1 >= 0
        && position.1 < rows.len() as i64
        && position.0 >= 0
        && position.0 < rows[0].len() as i64
}

fn print_rows(rows: &Vec<Vec<String>>) {
    for row in rows {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn get_antinodes(position1: &(i64, i64), position2: &(i64, i64), resonant: bool, rows: &Vec<Vec<String>>) -> Vec<(i64, i64)> {
    let mut antinodes = vec![];
    let x_difference = position1.0 - position2.0;
    let y_difference = position1.1 - position2.1;

    let mut first = (position1.0 + x_difference, position1.1 + y_difference);
    let mut second = (position2.0 - x_difference, position2.1 - y_difference);

    if !resonant {
        if position_in_bounds(&first, rows) { antinodes.push(first); }
        if position_in_bounds(&second, rows) { antinodes.push(second); }
        return antinodes;
    }

    while position_in_bounds(&first, rows) {
        antinodes.push(first);
        first = (first.0 + x_difference, first.1 + y_difference);
    }

    while position_in_bounds(&second, rows) {
        antinodes.push(second);
        second = (second.0 - x_difference, second.1 - y_difference);
    }

    antinodes
}

fn count_antinodes(puzzle: &str, resonate: bool) -> i64 {
    let mut rows = parse_string(puzzle, r"(.)").expect("Error parsing puzzle");
    let mut frequencies = HashMap::new();
    let mut antinodes = HashSet::new();

    for (y, row) in rows.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == "." {
                continue;
            }
            let entry = frequencies.entry(cell).or_insert(vec![]);
            entry.push((x as i64, y as i64));
            if resonate { antinodes.insert((x as i64, y as i64)); }
        }
    }

    for (_, positions) in &frequencies {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let new_antinodes = get_antinodes(&positions[i], &positions[j], resonate, &rows);
                for antinode in new_antinodes {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    for node in antinodes.iter() {
        rows[node.1 as usize][node.0 as usize] = "#".to_string();
    }

    print_rows(&rows);

    antinodes.len() as i64
}

fn main() {
    println!("Part 1: {}", count_antinodes(include_str!("data.txt"), false));
    println!("Part 2: {}", count_antinodes(include_str!("data.txt"), true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(count_antinodes(include_str!("test.txt"), false), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(count_antinodes(include_str!("test.txt"), true), 34);
    }
}
