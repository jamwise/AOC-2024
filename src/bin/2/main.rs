use aoc_2024::{log_output, parse_csv_by_row};

#[derive(PartialEq, Debug)]
enum Status {
    Safe,
    Unsafe,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Undefined,
    Ascending,
    Descending,
}

fn validate_report(report: &[i64], dampen: bool) -> Status {
    let mut prev_direction = Direction::Undefined;

    for i in 0..report.len() {
        if i == 0 {
            continue;
        }
        let left = report[i - 1];
        let right = report[i];
        let difference = right - left;

        let absolute = difference.abs();
        let current_direction = match difference > 0 {
            true => Direction::Ascending,
            false => Direction::Descending,
        };

        if (prev_direction != Direction::Undefined && prev_direction != current_direction)
            || absolute < 1
            || absolute > 3
        {
            if dampen {
                for j in 0..report.len() {
                    let mut cloned_report = report.to_vec();
                    cloned_report.remove(j);
                    if validate_report(&cloned_report, false) == Status::Safe {
                        return Status::Safe;
                    }
                }
                return Status::Unsafe;
            } else {
                return Status::Unsafe;
            }
        }

        prev_direction = current_direction;
    }

    Status::Safe
}

fn number_of_safe_reports(csv_string: &str, apply_dampener: bool) -> (i64, Vec<Status>) {
    let reports = parse_csv_by_row::<i64>(csv_string);
    let mut statuses = Vec::new();

    for report in reports {
        statuses.push(validate_report(&report, apply_dampener));
    }

    // get the number of statuses that are safe:
    let number = statuses
        .iter()
        .filter(|status| **status == Status::Safe)
        .count() as i64;

    (number, statuses)
}

fn main() {
    log_output(
        1,
        || number_of_safe_reports(include_str!("data.csv"), false).0
    );

    log_output(
        2,
        || number_of_safe_reports(include_str!("data.csv"), true).0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_safe_reports() {
        let (number, statuses) = number_of_safe_reports(include_str!("test.csv"), false);
        assert_eq!(number, 2);
        assert_eq!(
            statuses,
            vec![
                Status::Safe,
                Status::Unsafe,
                Status::Unsafe,
                Status::Unsafe,
                Status::Unsafe,
                Status::Safe
            ]
        );
    }

    #[test]
    fn test_number_of_safe_reports_dampened() {
        let (number, statuses) = number_of_safe_reports(include_str!("test.csv"), true);
        assert_eq!(number, 4);
        assert_eq!(
            statuses,
            vec![
                Status::Safe,
                Status::Unsafe,
                Status::Unsafe,
                Status::Safe,
                Status::Safe,
                Status::Safe
            ]
        );
    }
}
