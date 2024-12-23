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

    let profits = calc_profit(&initial_values);
    let max_bananas = profits.iter().max().unwrap();
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

fn calc_profit(initial_values: &Vec<u32>) -> [u16; 160000] {
    let mut profits = [0; 160000];
    for &value in initial_values.iter() {
        let mut seen_keys = [-1; 160000];
        let mut current_n = value;
        let mut last_p;
        let mut current_p = price(value);
        let mut key = 0;
        for m in 0..2000 {
            current_n = next(current_n);
            last_p = current_p;
            current_p = price(current_n);
            key = (key * 20 + ((current_p - last_p) + 9) as usize) % 160000;
            if m > 2 && seen_keys[key] == -1 {
                seen_keys[key] = current_p;
                profits[key] += current_p as u16;
            }
        }
    }
    profits
}

fn price(input: u32) -> i8 {
    (input % 10) as i8
}