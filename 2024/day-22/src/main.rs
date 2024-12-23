use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let initial_values = input_file.lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u32>>();

    let sum = initial_values.iter().fold(0, |sum, &n| {
        let mut num = n;
        for _ in 0..2000 {
            num = next(num);
        }
        sum + num as u64
    });
    println!("{}", sum);

    let profit_map = calc_profit(&initial_values);
    let mut max_bananas = 0;
    for (_, &bananas) in profit_map.iter() {
        if bananas > max_bananas {
            max_bananas = bananas;
        }
    }
    println!("{}", max_bananas);
}

fn next(input: u32) -> u32 {
    let a = prune(mix(input, input << 6));
    let b = prune(mix(a, a >> 5));
    prune(mix(b, b << 11))
}

fn mix(a: u32, b: u32) -> u32 {
    a ^ b
}

fn prune(a: u32) -> u32 {
    a & 0xFFFFFF
}

fn calc_profit(initial_values: &Vec<u32>) -> HashMap<u32, u16> {
    let mut profit_map = HashMap::new();
    for &value in initial_values.iter() {
        let mut current_n = value;
        let mut last_p;
        let mut current_p = price(value);
        let mut i;
        let mut j= 0;
        let mut k= 0;
        let mut l= 0;
        let mut seen_keys = HashSet::new();
        for m in 0..2000 {
            current_n = next(current_n);
            last_p = current_p;
            current_p = price(current_n);
            i = j;
            j = k;
            k = l;
            l = current_p - last_p;
            let key = gen_key(i as u8, j as u8, k as u8, l as u8);
            if m > 2 && !seen_keys.contains(&key) {
                seen_keys.insert(key);
                if profit_map.contains_key(&key) {
                    let total = profit_map.get(&key).unwrap();
                    profit_map.insert(key, total + current_p as u16);
                } else {
                    profit_map.insert(key, current_p as u16);
                }
            }
        }
    }
    profit_map
}

fn gen_key(i: u8, j: u8, k: u8, l: u8) -> u32 {
    ((i as u32) << 24) + ((j as u32) << 16) + ((k as u32) << 8) + l as u32
}

fn price(input: u32) -> i8 {
    (input % 10) as i8
}