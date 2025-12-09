use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut moves = Vec::new();
    for line in input_file.lines() {
        let direction = &line[0..1];
        let amount = line[1..].parse::<usize>().unwrap();
        moves.push((direction, amount));
    }

    let mut dial = 50;
    let mut count1 = 0;
    let mut count2 = 0;
    for (direction, amount) in moves {
        let mut spins = amount / 100;
        let remainder = amount % 100;
        match direction {
            "L" => {
                if remainder > dial {
                    if dial > 0 {
                        spins += 1;
                    }
                    dial = dial + 100 - remainder;
                } else {
                    dial -= remainder;
                }
                if dial == 0 {
                    spins += 1;
                }
            },
            "R" => {
                dial += remainder;
                if dial > 99 {
                    dial %= 100;
                    spins += 1;
                }
            },
            _ => unreachable!()
        }
        if dial == 0 {
            count1 += 1;
        }
        count2 += spins;
    }
    println!("{}", count1);
    println!("{}", count2);
}
