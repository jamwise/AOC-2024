use aoc_2024::parse_csv_by_column;

fn find_distance(csv_string: &str) -> i64 {
    let data = parse_csv_by_column::<i64>(csv_string);
    let mut left = data[0].clone();
    let mut right = data[1].clone();
    let mut distance: i64 = 0;
    left.sort();
    right.sort();

    for i in 0..left.len() {
        distance += (left[i] - right[i]).abs();
    }
    distance
}

fn calculate_similarity(csv_string: &str) -> i64 {
    let data = parse_csv_by_column::<i64>(csv_string);
    let left = data[0].clone();
    let right = data[1].clone();

    let mut similarity: i64 = 0;
    for i in 0..left.len() {
        let mut count: i64 = 0;
        for j in 0..right.len() {
            if left[i] == right[j] {
                count += 1;
            }
        }
        similarity += left[i] * count;
    }
    similarity
}

fn main() {
    println!("Part 1: {}", find_distance(include_str!("data.csv")));
    println!("Part 2: {}", calculate_similarity(include_str!("data.csv")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_distance() {
        assert_eq!(find_distance(include_str!("test.csv")), 11);
    }

    #[test]
    fn test_calculate_similarity() {
        assert_eq!(calculate_similarity(include_str!("test.csv")), 31);
    }
}
