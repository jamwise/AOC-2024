use fancy_regex::Regex;
use aoc_2024::log_output;

fn fix_corrupted_logic(memory: &str) -> i64 {
    let split_regex = r"(?m)(do\(\)|don't\(\))";
    let re = Regex::new(split_regex).unwrap();
    let mut total = 0;

    let parts: Vec<&str> = re.split(memory).map(|x| x.unwrap()).collect();
    let logic: Vec<bool> = re.find_iter(memory)
        .filter_map(Result::ok)
        .map(|m| {
            let string = m.as_str();
            if string == "do()" {
                true
            } else {
                false
            }
        })
        .collect();

    for i in 0..parts.len() {
        let part = parts[i];
        let do_it = if i == 0 { true } else { logic[i - 1] };
        if do_it {
            total += fix_corrupted_memory(part);
        }
    }

    total
}

fn fix_corrupted_memory(memory: &str) -> i64 {
    let regex_string = r"(?m)mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(regex_string).unwrap();

    let mut total = 0;
    for result in re.captures_iter(memory) {
        let captures = result.expect("Error running regex");
        let left: i64 = captures.get(1).expect("No group").as_str().parse().unwrap();
        let right: i64 = captures.get(2).expect("No group").as_str().parse().unwrap();
        total += left * right;
    }

    total
}

fn main() {
    log_output(
        1,
        || fix_corrupted_memory(include_str!("data.txt"))
    );
    log_output(
        2,
        || fix_corrupted_logic(include_str!("data.txt"))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_corrupted_memory() {
        assert_eq!(fix_corrupted_memory(include_str!("test1.txt")), 161);
    }

    #[test]
    fn test_fix_corrupted_memory_conditionals() {
        assert_eq!(fix_corrupted_logic(include_str!("test2.txt")), 48);
    }
}
