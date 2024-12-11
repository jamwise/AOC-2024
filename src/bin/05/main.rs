use aoc_2024::{log_output, parse_string};
use std::cmp::Ordering;
use std::collections::HashMap;

fn part1(rules: &str, updates: &str) -> i64 {
    let rule_rows: Vec<Vec<i64>> = parse_string(rules, vec!["|"]);
    let update_rows: Vec<Vec<i64>> = parse_string(updates, vec![","]);

    let rules_hash = rule_rows.iter().fold(HashMap::new(), |mut acc, row| {
        let entry = acc.entry(row[0].clone()).or_insert(vec![]);
        entry.push(row[1].clone());
        acc
    });

    let mut middle_pages: Vec<i64> = Vec::new();

    'update: for update in update_rows {
        let mut accumulated_page_numbers = Vec::new();
        for page_number in &update {
            if let Some(pages) = rules_hash.get(page_number) {
                if pages
                    .iter()
                    .any(|page| accumulated_page_numbers.contains(page))
                {
                    continue 'update;
                }
            }
            accumulated_page_numbers.push(page_number.clone());
        }
        let middle_page = update[update.len() / 2].clone();
        middle_pages.push(middle_page);
    }

    middle_pages.iter().sum()
}

fn part2(rules: &str, updates: &str) -> i64 {
    let rule_rows: Vec<Vec<i64>> = parse_string(rules, vec!["|"]);
    let update_rows: Vec<Vec<i64>> = parse_string(updates, vec![","]);

    let rules_hash = rule_rows.iter().fold(HashMap::new(), |mut acc, row| {
        let entry = acc.entry(row[0].clone()).or_insert(vec![]);
        entry.push(row[1].clone());
        acc
    });

    let mut middle_pages: Vec<i64> = Vec::new();

    for update in update_rows {
        let mut sorted_update = update.clone();
        let mut sorted = false;
        sorted_update.sort_by(|a, b| {
            if rules_hash
                .get(a)
                .map_or(false, |rule_vec| rule_vec.contains(b))
            {
                sorted = true;
                return Ordering::Less;
            }
            if rules_hash
                .get(b)
                .map_or(false, |rule_vec| rule_vec.contains(b))
            {
                sorted = true;
                return Ordering::Greater;
            }
            Ordering::Equal
        });
        if sorted {
            let middle_page = sorted_update[sorted_update.len() / 2].clone();
            middle_pages.push(middle_page);
        }
    }

    middle_pages.iter().sum()
}

fn main() {
    log_output(1, || {
        part1(
            include_str!("data_rules.txt"),
            include_str!("data_updates.txt"),
        )
    });
    log_output(2, || {
        part2(
            include_str!("data_rules.txt"),
            include_str!("data_updates.txt"),
        )
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                include_str!("test_rules.txt"),
                include_str!("test_updates.txt")
            ),
            143
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                include_str!("test_rules.txt"),
                include_str!("test_updates.txt")
            ),
            123
        );
    }
}
