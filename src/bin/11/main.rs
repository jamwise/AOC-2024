use cached::proc_macro::cached;

use aoc_2024::{log_output, parse_string};

#[cached]
fn process_stone(stone: usize, iteration: usize, iterations: usize) -> usize {
    if iteration == iterations {
        return 1;
    }
    if stone == 0 {
        return process_stone(1, iteration + 1, iterations);
    }

    let digit_count = (stone as f64).log10().floor() as i32 + 1;
    let divisor = 10_usize.pow((digit_count / 2) as u32);

    if digit_count % 2 == 0 {
        return process_stone(stone % divisor, iteration + 1, iterations)
            + process_stone(stone / divisor, iteration + 1, iterations);
    }

    process_stone(stone * 2024, iteration + 1, iterations)
}

fn process_stones(puzzle: &str, iterations: usize) -> usize {
    let puzzle: Vec<Vec<usize>> = parse_string(puzzle, vec![' ']);
    let stones = puzzle[0].clone();

    let mut total = 0;

    for stone in stones.iter() {
        total += process_stone(*stone, 0, iterations);
    }

    total
}

fn main() {
    log_output(1, || process_stones(include_str!("data.txt"), 25));
    log_output(2, || process_stones(include_str!("data.txt"), 75));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(process_stones(include_str!("test.txt"), 25), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process_stones(include_str!("test.txt"), 75), 65601038650482);
    }
}
