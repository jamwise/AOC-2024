use std::collections::{HashMap, HashSet};

use aoc_2024::{log_output, parse_string};

#[derive(Debug)]
struct Plots {
    edges: HashMap<(usize, usize), HashSet<(usize, usize)>>,
    directions: [(isize, isize); 4],
    visited: HashSet<(usize, usize)>,
    plots: Vec<(usize, usize, HashSet<(usize, (usize, usize))>)>,
}

impl Plots {
    fn new(puzzle: Vec<Vec<String>>) -> Self {
        let mut plots_struct = Self {
            directions: [(0, -1), (1, 0), (0, 1), (-1, 0)],
            edges: HashMap::new(),
            visited: HashSet::new(),
            plots: Vec::new(),
        };
        plots_struct.create_edges(&puzzle);
        plots_struct.plots = plots_struct.get_plots(puzzle);

        plots_struct
    }

    fn create_edges(&mut self, puzzle: &Vec<Vec<String>>) {
        let height = puzzle.len();
        let width = puzzle[0].len();

        for y in 0..height {
            for x in 0..width {
                let crop = &puzzle[y][x];
                for (dx, dy) in self.directions.iter() {
                    let next_position = (x.wrapping_add_signed(*dx), y.wrapping_add_signed(*dy));
                    if next_position.0 < width && next_position.1 < height {
                        let next_crop = &puzzle[next_position.1][next_position.0];
                        if crop == next_crop {
                            let plot = self.edges.entry((x, y)).or_insert(HashSet::new());
                            plot.insert(next_position);
                        }
                    }
                }
            }
        }
    }

    fn expand(
        &mut self,
        plot: &(usize, usize),
    ) -> (usize, usize, HashSet<(usize, (usize, usize))>) {
        self.visited.insert(plot.clone());
        let neighbors = self.edges.get(&plot).unwrap_or(&HashSet::new()).clone();

        let mut borders: HashSet<(usize, (usize, usize))> = HashSet::from_iter(vec![
            (0, plot.clone()),
            (1, plot.clone()),
            (2, plot.clone()),
            (3, plot.clone()),
        ]);

        let mut surface = 1;
        let mut border_count = 4 - neighbors.len();

        for neighbor in &neighbors {
            let direction = (
                neighbor.0 as isize - plot.0 as isize,
                neighbor.1 as isize - plot.1 as isize,
            );
            let direction = self
                .directions
                .iter()
                .position(|&x| x == direction)
                .unwrap();

            borders.remove(&(direction, plot.clone()));

            if self.visited.contains(neighbor) {
                continue;
            }
            let (neighbor_surface, neighbor_border, neighbor_borders) = self.expand(neighbor);
            surface += neighbor_surface;
            border_count += neighbor_border;
            borders.extend(neighbor_borders);
        }

        (surface, border_count, borders)
    }

    fn get_plots(&mut self, puzzle: Vec<Vec<String>>) -> Vec<(usize, usize, HashSet<(usize, (usize, usize))>)> {
        let height = puzzle.len();
        let width = puzzle[0].len();
        let mut plots = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let position = (x, y);

                if self.visited.contains(&position) {
                    continue;
                }
                if position.0 < width && position.1 < height {
                    plots.push(self.expand(&position).clone())
                }
            }
        }
        plots
    }
}

fn part1(puzzle: &str) -> usize {
    let puzzle: Vec<Vec<String>> = parse_string(puzzle, vec![]);

    let plots = Plots::new(puzzle);
    let mut total = 0;

    for (surface, border, _) in plots.plots.iter() {
        total += surface * border;
    }

    total
}

fn part2(puzzle: &str) -> usize {
    let puzzle: Vec<Vec<String>> = parse_string(puzzle, vec![]);

    let plots = Plots::new(puzzle);
    let mut total = 0;

    for (surface, _, borders) in plots.plots.iter() {       
        let borders: HashSet<(usize, (usize, usize))> = borders.iter().filter(|(direction, plot)| {
            if *direction == 0 || *direction == 2 {
                !borders.contains(&(*direction, (plot.0.wrapping_add_signed(-1), plot.1)))
            } else {
                !borders.contains(&(*direction, (plot.0, plot.1.wrapping_add_signed(-1))))
            }
        }).copied().collect();

        total += surface * borders.len();
    }

    total
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
        assert_eq!(part1(include_str!("test.txt")), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test.txt")), 1206);
    }
}
