use std::collections::HashMap;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut lines = input_file.lines();
    let towels = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
    lines.next();
    let mut patterns = Vec::new();
    let mut next_line = lines.next();
    while next_line.is_some() {
        patterns.push(next_line.unwrap());
        next_line = lines.next();
    }

    let possible = patterns.iter().fold(0, |sum, pattern| {
        if check_pattern(pattern, &towels) {
            sum + 1
        } else {
            sum
        }
    });
    println!("{}", possible);

    let mut pattern_map = HashMap::new();
    let total = patterns.iter().fold(0, |sum, pattern| {
        sum + count_patterns(pattern, &towels, &mut pattern_map)
    });
    println!("{}", total);
}

fn check_pattern(pattern: &str, towels: &Vec<&str>) -> bool {
    towels.iter().any(|&towel| {
        pattern == towel || (pattern.starts_with(towel) && check_pattern(&pattern[towel.len()..], towels))
    })
}

fn count_patterns<'a>(pattern: &'a str, towels: &Vec<&str>, pattern_map: &mut HashMap<&'a str, u64>) -> u64 {
    let previous_count = pattern_map.get(pattern);
    if previous_count.is_some() {
        *previous_count.unwrap()
    } else {
        let count = towels.iter().fold(0, |sum, &towel| {
            if pattern == towel {
                sum + 1
            } else if pattern.starts_with(towel) {
                sum + count_patterns(&pattern[towel.len()..], towels, pattern_map)
            } else {
                sum
            }
        });
        pattern_map.insert(pattern, count);
        count
    }
}
