use aoc_2024::{log_output, parse_string};

fn get_combo(a: (isize, isize), b: (isize, isize), target: (isize, isize)) -> Option<(isize, isize)> {
    let A = (target.0 * b.1 - target.1 * b.0) / (a.0 * b.1 - a.1 * b.0);
    let B = (target.0 - a.0 * A) / b.0;
    
    if A * a.0 + B * b.0 != target.0 || A * a.1 + B * b.1 != target.1 {
        return None;
    }
    
    Some((A, B))
}

fn get_tokens(puzzle: &str, increase: isize) -> isize {
    let puzzle: Vec<Vec<String>> = parse_string(puzzle, vec!['+', ',', ' ', '=']);
    let mut total = 0;

    for chunk in puzzle.chunks(4) {
        let ax: isize = chunk[0][3].parse().unwrap();
        let ay: isize = chunk[0][5].parse().unwrap();
        let bx: isize = chunk[1][3].parse().unwrap();
        let by: isize = chunk[1][5].parse().unwrap();

        let x: isize = chunk[2][2].parse().unwrap();
        let y: isize = chunk[2][4].parse().unwrap();

        if let Some((A, B)) = get_combo((ax, ay), (bx, by), (x + increase, y + increase)) {
            total += A * 3 + B;
        }
    }

    total
}

fn part1(puzzle: &str) -> isize {
    get_tokens(puzzle, 0)
}

fn part2(puzzle: &str) -> isize {
    get_tokens(puzzle, 10000000000000)
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
        assert_eq!(part1(include_str!("test.txt")), 480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test.txt")), 0);
    }
}
