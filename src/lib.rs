pub fn parse_csv_by_column(csv_string: &str) -> Vec<Vec<i64>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_string.as_bytes());

    let mut data: Vec<Vec<i64>> = Vec::new();

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

pub fn parse_csv_by_row(csv_string: &str) -> Vec<Vec<i64>> {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_reader(csv_string.as_bytes());

    let mut data: Vec<Vec<i64>> = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let mut row: Vec<i64> = Vec::new();
        for i in 0..record.len() {
            row.push(record[i].parse().unwrap());
        }
        data.push(row);
    }

    data
}
