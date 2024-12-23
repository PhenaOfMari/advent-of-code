use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let initial_values = input_file.lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u64>>();

    let sum = initial_values.iter().fold(0, |sum, &n| {
        let mut num = n;
        for _ in 0..2000 {
            num = next(num);
        }
        sum + num
    });
    println!("{}", sum);

    let (key_set, price_map) = build_price_data(&initial_values);
    let mut max_bananas = 0;
    for key in key_set.iter() {
        let bananas = price_map.iter().fold(0, |sum, (_, map)| {
            match map.get(key) {
                Some(&p) => sum + p,
                None => sum
            }
        });
        max_bananas = max_bananas.max(bananas);
    }
    println!("{}", max_bananas);
}

fn next(input: u64) -> u64 {
    let a = prune(mix(input, input * 64));
    let b = prune(mix(a, a / 32));
    prune(mix(b, b * 2048))
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(a: u64) -> u64 {
    a % 16777216
}

fn build_price_data(initial_values: &Vec<u64>) -> (HashSet<u32>, HashMap<u64, HashMap<u32, u16>>) {
    let mut key_set = HashSet::new();
    let mut price_map = HashMap::new();
    for &value in initial_values.iter() {
        let mut current_n = value;
        let mut last_p;
        let mut current_p = price(value);
        let mut prices = HashMap::new();
        let mut i;
        let mut j= 0;
        let mut k= 0;
        let mut l= 0;
        for m in 0..2000 {
            current_n = next(current_n);
            last_p = current_p;
            current_p = price(current_n);
            i = j;
            j = k;
            k = l;
            l = current_p - last_p;
            let key = gen_key(i as u8, j as u8, k as u8, l as u8);
            key_set.insert(key);
            if m > 2 && !prices.contains_key(&key) {
                prices.insert(key, current_p as u16);
            }
        }
        price_map.insert(value, prices);
    }
    (key_set, price_map)
}

fn gen_key(i: u8, j: u8, k: u8, l: u8) -> u32 {
    ((i as u32) << 24) + ((j as u32) << 16) + ((k as u32) << 8) + l as u32
}

fn price(input: u64) -> i8 {
    (input % 10) as i8
}