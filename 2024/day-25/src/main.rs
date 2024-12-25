use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    let mut current = [0u8; 5];
    let mut is_lock = false;
    let mut line_num: u8 = 0;
    for line in input_file.lines() {
        line_num += 1;
        if line.is_empty() {
            line_num = 0;
            if is_lock {
                locks.push(current);
            } else {
                keys.push(current);
            }
            current = [0u8; 5];
        } else if line_num == 1 {
            is_lock = line == "#####";
        } else if line_num < 7 {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    current[i] += 1;
                }
            }
        }
    }
    if is_lock {
        locks.push(current);
    } else {
        keys.push(current);
    }

    let fits = locks.iter().fold(0, |sum_l, lock| {
        sum_l + keys.iter().fold(0, |sum_k, key| {
            let key_fits = lock[0] + key[0] < 6
                && lock[1] + key[1] < 6
                && lock[2] + key[2] < 6
                && lock[3] + key[3] < 6
                && lock[4] + key[4] < 6;
            sum_k + u16::from(key_fits)
        })
    });
    println!("{}", fits);
}
