use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct Data {
    data: Vec<Vec<isize>>,
    test: Vec<Vec<isize>>,
    test_1_result: isize,
    test_2_result: isize,
}

fn find_distance(lists: &Vec<Vec<isize>>) -> isize {
    let mut distance: isize = 0;
    let mut list1 = lists[0].clone();
    let mut list2 = lists[1].clone();
    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        distance += (list1[i] - list2[i]).abs();
    }
    distance
}

fn calculate_similarity(lists: Vec<Vec<isize>>) -> isize {
    let mut similarity: isize = 0;
    for i in 0..lists[0].len() {
        let mut count: isize = 0;
        for j in 0..lists[1].len() {
            if lists[0][i] == lists[1][j] {
                count += 1;
            }
        }
        similarity += lists[0][i] * count;
    }
    similarity
}

fn main() {
    let data_str = include_str!("data.toml");
    let data: Data = toml::from_str(&data_str).unwrap();
    println!("Part 1: {}", find_distance(&data.data));
    println!("Part 2: {}", calculate_similarity(data.data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_distance() {
        let data_str = include_str!("data.toml");
        let data: Data = toml::from_str(&data_str).unwrap();
        assert_eq!(find_distance(&data.test), data.test_1_result);
    }

    #[test]
    fn test_calculate_similarity() {
        let data_str = include_str!("data.toml");
        let data: Data = toml::from_str(&data_str).unwrap();
        assert_eq!(calculate_similarity(data.test), data.test_2_result);
    }
}
