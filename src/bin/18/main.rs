use aoc_2024::{log_output, parse_string};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

type Point = (usize, usize);

pub fn find_shortest_path(
    graph: &HashMap<Point, Vec<Point>>,
    start_point: Point,
    end_point: Point,
) -> Option<(Vec<Vec<Point>>, usize)> {
    let mut distances: HashMap<Point, usize> = HashMap::new();
    let mut previous: HashMap<Point, Point> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    distances.insert(start_point, 0);
    heap.push(Reverse((0, start_point)));
    while let Some(Reverse((cost, position))) = heap.pop() {
        if position == end_point {
            let mut path = vec![position];
            let mut current = position;
            while let Some(&prev) = previous.get(&current) {
                path.push(prev);
                current = prev;
            }
            path.reverse();
            return Some((vec![path], cost));
        }
        if visited.contains(&position) {
            continue;
        }
        visited.insert(position);
        if let Some(neighbors) = graph.get(&position) {
            for &next in neighbors {
                let new_cost = cost + 1;

                if !distances.contains_key(&next) || new_cost < *distances.get(&next).unwrap() {
                    distances.insert(next, new_cost);
                    previous.insert(next, position);
                    heap.push(Reverse((new_cost, next)));
                }
            }
        }
    }
    None
}

fn create_graph(
    size: usize,
    obstacles: &Vec<Vec<usize>>,
    limit: usize,
) -> HashMap<Point, Vec<Point>> {
    let mut map = vec![vec!['.'; size + 1]; size + 1];
    let mut graph = HashMap::new();
    for (i, obstacle) in obstacles.iter().enumerate() {
        if i == limit {
            break;
        }
        map[obstacle[1]][obstacle[0]] = '#';
    }
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let node = map[row][col];
            if node == '#' {
                continue;
            }
            let mut node_and_neighbors = vec![];
            if row > 0 && map[row - 1][col] != '#' {
                node_and_neighbors.push((col, row - 1));
            }
            if col < size && map[row][col + 1] != '#' {
                node_and_neighbors.push((col + 1, row));
            }
            if row < size && map[row + 1][col] != '#' {
                node_and_neighbors.push((col, row + 1));
            }
            if col > 0 && map[row][col - 1] != '#' {
                node_and_neighbors.push((col - 1, row));
            }
            graph.insert((col, row), node_and_neighbors);
        }
    }
    graph
}

fn part1(puzzle: &str, size: usize, limit: usize) -> usize {
    let puzzle: Vec<Vec<usize>> = parse_string(puzzle, vec![',']);
    let graph = create_graph(size, &puzzle, limit);
    let path = find_shortest_path(&graph, (0, 0), (size, size)).unwrap();

    path.1
}

fn part2(puzzle: &str, size: usize) -> String {
    let puzzle: Vec<Vec<usize>> = parse_string(puzzle, vec![',']);
    let mut visited_indices = HashSet::new();
    let mut lower_limit = 0;
    let mut upper_limit = puzzle.len();
    let mut current_index = upper_limit;

    while !visited_indices.contains(&current_index) {
        visited_indices.insert(current_index);
        let graph = create_graph(size, &puzzle, current_index);
        let path = find_shortest_path(&graph, (0, 0), (size, size));

        if path.is_none() {
            upper_limit = current_index;
        } else {
            lower_limit = current_index;
        }
        current_index = (upper_limit + lower_limit) / 2;
    }
    puzzle[current_index]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn main() {
    log_output(1, || part1(include_str!("data.txt"), 70, 1024));
    log_output(2, || part2(include_str!("data.txt"), 70));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("test.txt"), 6, 1024), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test.txt")), 0);
    }
}
