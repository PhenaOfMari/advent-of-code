use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut char_matrix = Vec::new();
    let mut ops = Vec::new();
    for line in input_file.lines() {
        if line.starts_with("+") || line.starts_with("*") {
            ops = line.trim().split_whitespace().collect();
        } else {
            char_matrix.push(line.chars().collect::<Vec<char>>());
        }
    }
    let col_len = char_matrix.len();
    let row_len = char_matrix[0].len();

    let mut sum1 = 0;
    let mut i = 0;
    let mut op_num = 0;
    let mut operands = vec![String::new(); col_len];
    while i <= row_len {
        if i == row_len || char_matrix.iter().all(|row| row[i] == ' ') {
            sum1 += fold_operands(&operands, ops[op_num]);
            op_num += 1;
            operands = vec![String::new(); col_len];
        } else {
            for j in 0..col_len {
                let char = char_matrix[j][i];
                if char != ' ' {
                    operands[j].push(char)
                }
            }
        }
        i += 1;
    }
    println!("{}", sum1);

    let mut sum2 = 0;
    i = 0;
    op_num = 0;
    operands = Vec::new();
    while i <= row_len {
        if i == row_len || char_matrix.iter().all(|row| row[i] == ' ') {
            sum2 += fold_operands(&operands, ops[op_num]);
            op_num += 1;
            operands.clear();
        } else {
            let operand = char_matrix.iter().fold(String::new(), |mut acc, row| {
                acc.push(row[i]);
                acc
            });
            operands.push(operand);
        }
        i += 1;
    }
    println!("{}", sum2);
}

fn fold_operands(operands: &[String], op: &str) -> usize {
    match op {
        "*" => operands.iter().fold(1, |acc, num| acc * num.trim().parse::<usize>().unwrap()),
        "+" => operands.iter().fold(0, |acc, num| acc + num.trim().parse::<usize>().unwrap()),
        _ => unreachable!(),
    }
}