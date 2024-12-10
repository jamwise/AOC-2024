use fancy_regex::Regex;
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

pub fn parse_string(
    input: &str,
    pattern: &str,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let regex = Regex::new(pattern)?;

    let result = input
        .lines()
        .map(|line| {
            let line_matches: Result<Vec<Vec<String>>, Box<dyn std::error::Error>> = regex
                .captures_iter(line)
                .map(
                    |capture_result| -> Result<Vec<String>, Box<dyn std::error::Error>> {
                        let captures = capture_result.unwrap();

                        Ok((1..captures.len())
                            .filter_map(|i| captures.get(i).map(|m| m.as_str().to_string()))
                            .collect())
                    },
                )
                .collect();


            line_matches.map(|matches| matches.into_iter().flatten().collect())
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

    Ok(result)
}

pub fn log_output<F>(part: usize, function: F) -> ()
where F: Fn() -> i64
{
    let start = Instant::now();
    let result = function();
    let duration = start.elapsed();
    println!("Part {}: {} in {:.1?}", part, result, duration);
}

pub fn print_rows<T>(rows: &Vec<Vec<T>>) -> ()
where T: std::fmt::Display 
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