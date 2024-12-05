use fancy_regex::Regex;

pub fn parse_csv_by_column<T: std::str::FromStr>(csv_string: &str) -> Vec<Vec<T>> 
where <T as std::str::FromStr>::Err: std::fmt::Debug {
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
where <T as std::str::FromStr>::Err: std::fmt::Debug {
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

pub fn parse_string(input: &str, pattern: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let regex = Regex::new(pattern)?;
    
    let result = input
        .lines()
        .map(|line| {
            if let Some(captures) = regex.captures(line).expect("No captures") {
                Ok((1..captures.len())
                    .map(|i| captures.get(i).unwrap().as_str().to_string())
                    .collect())
            } else {
                Ok(Vec::new())
            }
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

    Ok(result)
}