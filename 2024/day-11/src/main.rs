use std::collections::HashMap;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let stones = input_file.lines()
        .flat_map(|line| line.split_whitespace().map(|item| item.parse().unwrap()).collect::<Vec<u64>>())
        .collect::<Vec<u64>>();
    let mut blink_map = HashMap::new();

    let blinks_25 = stones.iter().fold(0, |sum, &stone| sum + count_stones(stone, 25, &mut blink_map));
    println!("{}", blinks_25);

    let blinks_75 = stones.iter().fold(0, |sum, &stone| sum + count_stones(stone, 75, &mut blink_map));
    println!("{}", blinks_75);
}

fn count_stones(value: u64, blinks: u64, result_lookup: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks == 0 {
        return 1
    }
    match result_lookup.get(&(value, blinks)) {
        Some(&count) => count,
        None => {
            let result = if value == 0 {
                count_stones(1, blinks - 1, result_lookup)
            } else {
                let digits = value.ilog10() + 1;
                if digits % 2 == 0 {
                    let split_power = digits / 2;
                    let left = value / 10u64.pow(split_power);
                    let right = value - left * 10u64.pow(split_power);
                    count_stones(left, blinks - 1, result_lookup) + count_stones(right, blinks - 1, result_lookup)
                } else {
                    count_stones(value * 2024, blinks - 1, result_lookup)
                }
            };
            result_lookup.insert((value, blinks), result);
            result
        }
    }
}
