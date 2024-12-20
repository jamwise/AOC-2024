use aoc_2024::{log_output, parse_string};
use std::collections::{HashMap, HashSet};

fn can_make_design(
    design: &str,
    patterns: &HashSet<String>,
    memo: &mut HashMap<String, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(&result) = memo.get(design) {
        return result;
    }
    for pattern in patterns {
        if design.starts_with(pattern) {
            if can_make_design(&design[pattern.len()..], patterns, memo) {
                memo.insert(design.to_string(), true);
                return true;
            }
        }
    }
    memo.insert(design.to_string(), false);
    false
}

fn count_arrangements(
    design: &str,
    patterns: &HashSet<String>,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(&result) = memo.get(design) {
        return result;
    }
    let mut total = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            total += count_arrangements(&design[pattern.len()..], patterns, memo);
        }
    }
    memo.insert(design.to_string(), total);
    total
}

fn parse(puzzle: &str) -> (HashSet<String>, Vec<String>) {
    let puzzle: Vec<Vec<String>> = parse_string(puzzle, vec![',', ' ']);
    let patterns = HashSet::from_iter(puzzle[0].clone());
    let mut designs = puzzle.clone();
    designs.remove(0);
    let designs: Vec<String> = designs.into_iter().flatten().collect();
    let designs: Vec<String> = designs.into_iter().filter(|s| s != "").collect();

    (patterns, designs)
}

fn part1(puzzle: &str) -> usize {
    let (patterns, designs) = parse(puzzle);
    let mut memo = HashMap::new();

    designs
        .iter()
        .filter(|design| can_make_design(design, &patterns, &mut memo))
        .count()
}

fn part2(puzzle: &str) -> u64 {
    let (patterns, designs) = parse(puzzle);
    let mut memo = HashMap::new();

    designs
        .iter()
        .map(|design| count_arrangements(design, &patterns, &mut memo))
        .sum()
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
        assert_eq!(part1(include_str!("test.txt")), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test.txt")), 16);
    }
}
