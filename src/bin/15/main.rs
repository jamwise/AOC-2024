use aoc_2024::{log_output, parse_string};

fn part1(puzzle: &str) -> usize {
    let puzzle: Vec<Vec<String>> = parse_string(puzzle, vec![]);

    0
}

fn part2(puzzle: &str) -> usize {
    let puzzle: Vec<Vec<String>> = parse_string(puzzle, vec![]);
    
    0
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
        assert_eq!(part1(include_str!("test.txt")), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test.txt")), 0);
    }
}
