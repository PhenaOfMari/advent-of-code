use std::fs;
use std::ops::{Add, Mul};

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let equations = input_file.lines()
        .map(|line| {
            let mut split = line.split(':');
            let value = split.next().unwrap().parse().unwrap();
            let terms = split.next().unwrap().split_whitespace()
                .map(|item| item.parse().unwrap()).collect();
            (value, terms)
        })
        .collect();

    let add_mul_ops: [fn(u64, u64) -> u64; 2] = [u64::add, u64::mul];
    let add_mul = fold_equations(&equations, &add_mul_ops);
    println!("{}", add_mul);

    let concat_ops: [fn(u64, u64) -> u64; 3] = [u64::add, u64::mul, concat];
    let concat = fold_equations(&equations, &concat_ops);
    println!("{}", concat);
}

fn fold_equations(equations: &Vec<(u64, Vec<u64>)>, ops: &[fn(u64, u64) -> u64]) -> u64 {
    equations.iter().fold(0, |sum, equation| {
        let slice = equation.1.as_slice();
        if crunch(equation.0, slice[0], &slice[1..], ops) {
            sum + equation.0
        } else {
            sum
        }
    })
}

fn crunch(target: u64, head: u64, tail: &[u64], ops: &[fn(u64, u64) -> u64]) -> bool {
    match tail {
        [next, rest @ ..] => ops.iter().any(|op| crunch(target, op(head, *next), rest, ops)),
        [] => target == head
    }
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}
