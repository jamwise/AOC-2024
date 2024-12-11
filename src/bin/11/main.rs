use cached::proc_macro::cached;

use aoc_2024::{log_output, parse_string};

#[cached]
fn recurse(stone: usize, blink: usize, blinks: usize) -> usize {
    let digit_count = (stone as f64).log10().floor() as usize + 1;
    let next = blink + 1;

    if blink == blinks {
        1
    } else if stone == 0 {
        recurse(1, next, blinks)
    } else if digit_count % 2 == 0 {
        let divisor = 10_usize.pow((digit_count / 2) as u32);
        recurse(stone % divisor, next, blinks) + recurse(stone / divisor, next, blinks)
    } else {
        recurse(stone * 2024, next, blinks)
    }
}

fn process_stones(puzzle: &str, iterations: usize) -> usize {
    let puzzle: Vec<Vec<usize>> = parse_string(puzzle, vec![' ']);
    let stones = puzzle[0].clone();

    let mut total = 0;

    for stone in stones.iter() {
        total += recurse(*stone, 0, iterations);
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
