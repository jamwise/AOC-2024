use aoc_2024::{parse_string, log_output};

fn generate_combinations(chars: &Vec<char>, length: usize) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    let base = chars.len();
    let total_combinations = base.pow(length as u32);

    for i in 0..total_combinations {
        let mut current = Vec::with_capacity(length);
        let mut num = i;

        for _ in 0..length {
            let digit = num % base;
            current.push(chars[digit]);
            num /= base;
        }

        result.push(current);
    }

    result
}

fn get_result(puzzle: &str, add_pipe: bool) -> i64 {
    let mut total = 0;
    let rows: Vec<Vec<i64>> = parse_string(puzzle, vec![':', ' ']);
    let mut operators = vec!['+', '*'];
    if add_pipe {
        operators.push('|');
    }

    'row: for row in rows {
        let desired_total = row[0];
        let combinations = generate_combinations(&operators, row.len() - 2);

        for combination in combinations {
            let mut combination_total: i64 = row[1];

            for i in 2..row.len() {
                let operator = combination[i - 2];
                let number: i64 = row[i];

                match operator {
                    '+' => combination_total += number,
                    '*' => combination_total *= number,
                    '|' => {
                        let combined = format!("{}{}", combination_total, number);
                        combination_total = combined.parse().unwrap();
                    }
                    _ => panic!("Invalid operator"),
                }
            }

            if combination_total == desired_total {
                total += desired_total;
                continue 'row;
            }
        }
    }

    total
}

fn part1(puzzle: &str) -> i64 {
    get_result(puzzle, false)
}

fn part2(puzzle: &str) -> i64 {
    get_result(puzzle, true)
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
        assert_eq!(part1(include_str!("test.txt")), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test.txt")), 11387);
    }
}
