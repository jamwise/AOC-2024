use aoc_2024::parse_csv_by_row;

fn count_x_max(csv_string: &str) -> i64 {
    let directions: Vec<((i64, i64), (i64, i64))> = vec![
        ((1, -1), (-1, 1)), // NE, SW
        ((1, 1), (-1, -1)), // SE, NW
        ((-1, 1), (1, -1)), // SW, NE
        ((-1, -1), (1, 1)), // NW, SE
    ];

    let rows = parse_csv_by_row::<String>(csv_string);

    let mut total: i64 = 0;

    for y in 0..rows.len() {
        'x: for x in 0..rows[y].len() {
            if rows[x][y] != String::from('A') {
                continue;
            }
            let mut matches = 0;
            'directions: for (before, after) in directions.iter() {
                let before_x = (x as i64) + before.0;
                let before_y = (y as i64) + before.1;
                if before_x < 0
                    || before_y < 0
                    || before_x >= rows.len() as i64
                    || before_y >= rows[x].len() as i64
                {
                    continue 'directions;
                }
                let before_letter = rows[before_x as usize][before_y as usize].clone();
                if before_letter != String::from('M') {
                    continue 'directions;
                }

                let after_x = (x as i64) + after.0;
                let after_y = (y as i64) + after.1;
                if after_x < 0
                    || after_y < 0
                    || after_x >= rows.len() as i64
                    || after_y >= rows[x].len() as i64
                {
                    continue 'directions;
                }
                let after_letter = rows[after_x as usize][after_y as usize].clone();

                if after_letter != String::from('S') {
                    continue 'directions;
                }

                matches += 1;

                if matches > 1 {
                    total += 1;
                    continue 'x;
                }
            }
        }
    }

    total
}

fn count_xmas(csv_string: &str) -> i64 {
    let directions: Vec<(i64, i64)> = vec![
        (0, -1),  // N
        (1, 0),   // E
        (0, 1),   // S
        (-1, 0),  // W
        (1, -1),  // NE
        (1, 1),   // SE
        (-1, 1),  // SW
        (-1, -1), // NW
    ];

    let rows = parse_csv_by_row::<String>(csv_string);

    let to_find = vec![
        String::from('X'),
        String::from('M'),
        String::from('A'),
        String::from('S'),
    ];

    let to_find_len = to_find.len();

    let mut total: i64 = 0;

    for y in 0..rows.len() {
        for x in 0..rows[y].len() {
            if rows[x][y] != to_find[0] {
                continue;
            }
            'directions: for (x_direction, y_direction) in directions.iter() {
                for i in 1..to_find_len {
                    let new_x = (x as i64) + x_direction * (i as i64);
                    let new_y = (y as i64) + y_direction * (i as i64);

                    if new_x < 0
                        || new_y < 0
                        || new_x >= rows.len() as i64
                        || new_y >= rows[x].len() as i64
                    {
                        continue 'directions;
                    }

                    let letter = rows[new_x as usize][new_y as usize].clone();

                    if letter != to_find[i] {
                        continue 'directions;
                    }

                    if i == to_find_len - 1 {
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

fn main() {
    println!("Part 1: {}", count_xmas(include_str!("data.csv")));
    println!("Part 2: {}", count_x_max(include_str!("data.csv")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_xmas() {
        assert_eq!(count_xmas(include_str!("test.csv")), 18);
    }

    #[test]
    fn test_count_x_max() {
        assert_eq!(count_x_max(include_str!("test.csv")), 9);
    }
}
