use std::str::FromStr;
use std::time::Instant;

pub fn parse_csv_by_column<T: std::str::FromStr>(csv_string: &str) -> Vec<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_string.as_bytes());

    let mut data: Vec<Vec<T>> = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        for i in 0..record.len() {
            if data.len() <= i {
                data.push(Vec::new());
            }
            data[i].push(record[i].parse().unwrap());
        }
    }

    data
}

pub fn parse_csv_by_row<T: std::str::FromStr>(csv_string: &str) -> Vec<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_reader(csv_string.as_bytes());

    let mut data: Vec<Vec<T>> = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let mut row: Vec<T> = Vec::new();
        for i in 0..record.len() {
            row.push(record[i].parse().unwrap());
        }
        data.push(row);
    }

    data
}

pub fn parse_string<T>(input: &str, mode: Vec<&str>) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| match &mode.len() {
            0 => line
                .chars()
                .map(|c| c.to_string().parse::<T>().unwrap())
                .collect(),
            _ => {
                let mut split_result = vec![line.to_string()];
                for delimiter in mode.iter() {
                    split_result = split_result
                        .iter()
                        .flat_map(|s| s.split(delimiter))
                        .map(|s| s.to_string())
                        .collect();
                }
                split_result
                    .into_iter()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<T>().unwrap()) 
                    .collect()
            }
        })
        .collect()
}

pub fn log_output<F>(part: usize, function: F) -> ()
where
    F: Fn() -> i64,
{
    let start = Instant::now();
    let result = function();
    let duration = start.elapsed();
    println!("Part {}: {} in {:.1?}", part, result, duration);
}

pub fn print_rows<T>(rows: &Vec<Vec<T>>) -> ()
where
    T: std::fmt::Display,
{
    for row in rows {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn is_in_bounds<T, U>(position: &(U, U), grid: &Vec<Vec<T>>) -> bool
where
    U: std::cmp::PartialOrd<usize> + Into<usize> + Copy,
{
    position.1 >= 0 && position.1 < grid.len() && position.0 >= 0 && position.0 < grid[0].len()
}
