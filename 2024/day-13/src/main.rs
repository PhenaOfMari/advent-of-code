use std::fs;
use utils::cartesian::BigCartesian;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut lines = input_file.lines();
    let mut machines = Vec::new();

    let mut line = lines.next();
    while line.is_some() {
        let mut first = line.unwrap().split_whitespace();
        first.next();
        first.next();
        let a_x_str = first.next().unwrap();
        let a_x = a_x_str[2..a_x_str.len() - 1].parse::<i64>().unwrap();
        let a_y = first.next().unwrap()[2..].parse::<i64>().unwrap();
        let a = BigCartesian::new(a_x, a_y);
        let mut second = lines.next().unwrap().split_whitespace();
        second.next();
        second.next();
        let b_x_str = second.next().unwrap();
        let b_x = b_x_str[2..b_x_str.len() - 1].parse::<i64>().unwrap();
        let b_y = second.next().unwrap()[2..].parse::<i64>().unwrap();
        let b = BigCartesian::new(b_x, b_y);
        let mut third = lines.next().unwrap().split_whitespace();
        third.next();
        let prize_x_str = third.next().unwrap();
        let prize_x = prize_x_str[2..prize_x_str.len() - 1].parse::<i64>().unwrap();
        let prize_y = third.next().unwrap()[2..].parse::<i64>().unwrap();
        let prize = BigCartesian::new(prize_x, prize_y);

        machines.push((a, b, prize));

        lines.next();
        line = lines.next();
    }

    let min_tokens = machines.iter().fold(0, |sum, &machine| sum + calculate_tokens(machine));
    println!("{}", min_tokens);

    let offset = BigCartesian::new(10000000000000, 10000000000000);
    let min_tokens_offset = machines.iter().fold(0, |sum, &machine| {
        let offset_machine = (machine.0, machine.1, machine.2 + offset);
        sum + calculate_tokens(offset_machine)
    });
    println!("{}", min_tokens_offset);
}

fn calculate_tokens((a, b, p): (BigCartesian, BigCartesian, BigCartesian)) -> i64 {
    // [a.x b.x][a_count] = [p.x]
    // [a.y b.y][b_count]   [p.y]
    let det = a.x * b.y - a.y * b.x;
    let det_a = p.x * b.y - b.x * p.y;
    let det_b = a.x * p.y - p.x * a.y;

    if det_a % det == 0 && det_b % det == 0 {
        let a_count = det_a / det;
        let b_count = det_b / det;
        3 * a_count + 1 * b_count
    } else {
        0
    }
}
