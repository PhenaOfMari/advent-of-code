use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut lines = input_file.lines();
    let a = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let b = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let c = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    lines.next();
    let program = lines.next().unwrap()[9..].split(',').map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();

    let mut output = compute(&program, a, b, c);
    println!("{}", output.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","));

    let mut n = program.len() - 2;
    let mut i = 8u64.pow((n + 1) as u32);
    loop {
        output = compute(&program, i, b, c);
        while output[n] == program[n] && output[n+1] == program[n+1] {
            if n == 0 {
                break;
            } else {
                n -= 2;
            }
        }
        if n == 0 && output[n] == program[n] && output[n+1] == program[n+1] {
            break;
        }
        i += 8u64.pow(n as u32);
    }
    println!("{}", i);
}

fn compute(program: &Vec<u8>, a_start: u64, b_start: u64, c_start: u64) -> Vec<u8> {
    let mut a = a_start;
    let mut b = b_start;
    let mut c = c_start;
    let mut output = Vec::new();
    let mut pointer = 0;
    while pointer < program.len() {
        let opcode = program[pointer];
        let operand = program[pointer + 1];
        match opcode {
            0 => a = a / 2u64.pow(combo(operand, a, b, c) as u32),
            1 => b = b ^ operand as u64,
            2 => b = combo(operand, a, b, c) % 8,
            3 => if a != 0 {
                pointer = operand as usize;
                continue;
            },
            4 => b = b ^ c,
            5 => output.push((combo(operand, a, b, c) % 8) as u8),
            6 => b = a / 2u64.pow(combo(operand, a, b, c) as u32),
            7 => c = a / 2u64.pow(combo(operand, a, b, c) as u32),
            _ => unreachable!()
        }
        pointer += 2;
    }
    output
}

fn combo(operand: u8, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!()
    }
}
