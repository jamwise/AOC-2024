use aoc_2024::parse_string;

fn part1(puzzle: &str) -> i64 {
    let rows = parse_string(puzzle, r"(\d+)").expect("Error parsing puzzle");

    0
}

fn part2(puzzle: &str) -> i64 {
    let rows = parse_string(puzzle, r"(\d+)").expect("Error parsing puzzle");
    
    0
}

fn main() {
    println!("Part 1: {}", part1(include_str!("data.txt")));
    println!("Part 2: {}", part2(include_str!("data.txt")));
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
