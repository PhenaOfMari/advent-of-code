use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut banks = Vec::new();
    for line in input_file.lines() {
        let bank = line.chars().map(|c| c.to_string().parse().unwrap()).collect::<Vec<usize>>();
        banks.push(bank);
    }

    let mut sum1 = 0;
    let mut sum2 = 0;
    for bank in banks {
        sum1 += joltage::<2>(&bank);
        sum2 += joltage::<12>(&bank);
    }
    println!("{}", sum1);
    println!("{}", sum2);
}

fn joltage<const N: usize>(bank: &[usize]) -> usize {
    let mut vals = [0; N];
    let mut idx = 0;
    let len = bank.len();
    for i in 0..N {
        for j in idx..len - (N - 1 - i) {
            if bank[j] > vals[i] {
                vals[i] = bank[j];
                idx = j + 1;
            }
        }
    }
    vals.iter().rev().enumerate().fold(0, |sum, (i, &n)| sum + n * 10usize.pow(i as u32))
}
