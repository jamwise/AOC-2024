use aoc_2024::{log_output, parse_string};

use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::Reverse;

type Point = (usize, usize);

fn cost(prev: Point, current: Point, next: Point) -> u32 {
    let is_line =
        prev.0 == current.0 && current.0 == next.0 || prev.1 == current.1 && current.1 == next.1;
    if is_line {
        1
    } else {
        1001
    }
}

fn build_all_paths(
    current: Point,
    current_prev: Point,
    start: Point,
    previous_points: &HashMap<(Point, Point), HashSet<Point>>,
    path_so_far: Vec<Point>,
    all_paths: &mut Vec<Vec<Point>>
) {
    let mut new_path = path_so_far.clone();
    new_path.push(current);
    
    if current == start {
        new_path.reverse();
        all_paths.push(new_path);
        return;
    }
    
    if let Some(prev_points) = previous_points.get(&(current, current_prev)) {
        for &prev in prev_points {
            build_all_paths(current_prev, prev, start, previous_points, new_path.clone(), all_paths);
        }
    }
}

pub fn find_shortest_path(
    graph: &HashMap<Point, Vec<Point>>,
    start_point: Point,
    end_point: Point,
) -> Option<(Vec<Vec<Point>>, u32)> {
    let mut shortest_distances: HashMap<(Point, Point), u32> = HashMap::new();
    
    let mut previous_points: HashMap<(Point, Point), HashSet<Point>> = HashMap::new();
    
    let mut points_to_visit: BinaryHeap<(Reverse<u32>, Point, Point)> = BinaryHeap::new();
    
    shortest_distances.insert((start_point, (start_point.0 - 1, start_point.1)), 0);
    points_to_visit.push((Reverse(0), start_point, (start_point.0 - 1, start_point.1)));
    previous_points.insert((start_point, start_point), HashSet::new());

    let mut shortest_path_found = false;
    let mut final_distance = 0;

    while let Some((Reverse(current_distance), current_point, prev_point)) = points_to_visit.pop() {
        if shortest_path_found && current_distance > final_distance {
            break;
        }

        if current_point == end_point {
            shortest_path_found = true;
            final_distance = current_distance;
            continue;
        }

        if let Some(&best_known_distance) = shortest_distances.get(&(current_point, prev_point)) {
            if current_distance > best_known_distance {
                continue;
            }
        }

        for &neighbor_point in graph.get(&current_point).unwrap() {
            if neighbor_point == current_point {
                continue;
            }

            let segment_cost = cost(prev_point, current_point, neighbor_point);
            let total_distance = current_distance + segment_cost;

            match shortest_distances.get(&(neighbor_point, current_point)) {
                None => {
                    shortest_distances.insert((neighbor_point, current_point), total_distance);
                    let mut prev_set = HashSet::new();
                    prev_set.insert(prev_point);
                    previous_points.insert((neighbor_point, current_point), prev_set);
                    points_to_visit.push((Reverse(total_distance), neighbor_point, current_point));
                }
                Some(&known_distance) => {
                    if ((total_distance - known_distance) as f64).abs() < f64::EPSILON {
                        previous_points.get_mut(&(neighbor_point, current_point))
                            .unwrap()
                            .insert(prev_point);
                    } else if total_distance < known_distance {
                        shortest_distances.insert((neighbor_point, current_point), total_distance);
                        let mut prev_set = HashSet::new();
                        prev_set.insert(prev_point);
                        previous_points.insert((neighbor_point, current_point), prev_set);
                        points_to_visit.push((Reverse(total_distance), neighbor_point, current_point));
                    }
                }
            }
        }
    }

    if !shortest_path_found {
        return None;
    }

    let mut all_paths = Vec::new();
    
    for (&(pos, prev), &cost) in &shortest_distances {
        if pos == end_point && ((cost - final_distance) as f64).abs() < f64::EPSILON {
            build_all_paths(pos, prev, start_point, &previous_points, Vec::new(), &mut all_paths);
        }
    }

    Some((all_paths, final_distance))
}

fn create_graph(puzzle: &Vec<Vec<String>>) -> (Point, Point, HashMap<Point, Vec<Point>>) {
    let mut graph = HashMap::new();
    let mut start: Point = (0, 0);
    let mut end: Point = (0, 0);

    for row in 0..puzzle.len() {
        for col in 0..puzzle[row].len() {
            let node = &puzzle[row][col];
            if node == "#" {
                continue;
            }
            if node == "S" {
                start = (col, row);
            }
            if node == "E" {
                end = (col, row);
            }

            let mut node_and_neighbors = vec![];

            if puzzle[row - 1][col] != "#" {
                node_and_neighbors.push((col, row - 1));
            }
            if puzzle[row][col + 1] != "#" {
                node_and_neighbors.push((col + 1, row));
            }
            if puzzle[row + 1][col] != "#" {
                node_and_neighbors.push((col, row + 1));
            }
            if puzzle[row][col - 1] != "#" {
                node_and_neighbors.push((col - 1, row));
            }

            graph.insert((col, row), node_and_neighbors);
        }
    }

    (start, end, graph)
}

fn part1(puzzle: &str) -> u32 {
    let puzzle: Vec<Vec<String>> = parse_string(puzzle, vec![]);
    let (start, end, graph) = create_graph(&puzzle);

    let (_, cost) = find_shortest_path(&graph, start, end).unwrap_or((vec![], 0));

    cost
}

fn part2(puzzle: &str) -> usize {
    let puzzle: Vec<Vec<String>> = parse_string(puzzle, vec![]);
    let (start, end, graph) = create_graph(&puzzle);

    let result = find_shortest_path(&graph, start, end);

    let mut all_nodes = HashSet::new();
    if let Some((paths, _)) = result {
        for path in paths {
            for node in path {
                all_nodes.insert(node);
            }
        }
    }

    all_nodes.len()
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
        assert_eq!(part1(include_str!("test.txt")), 7036);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test.txt")), 45);
    }
}
