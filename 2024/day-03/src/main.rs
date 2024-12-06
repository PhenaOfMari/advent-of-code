extern crate core;

use std::fs;
use regex::{Captures, Regex};

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");

    let mul_pattern = Regex::new(r"mul\((?<a>[0-9]+),(?<b>[0-9]+)\)").unwrap();
    let mul_instructions = input_file.lines()
        .flat_map(|line| mul_pattern.captures_iter(line).collect::<Vec<Captures>>())
        .collect::<Vec<Captures>>();

    let mul_result = mul_instructions.iter().fold(0, |sum, capture| {
        let a = capture.name("a").unwrap().as_str().parse::<u32>().unwrap();
        let b = capture.name("b").unwrap().as_str().parse::<u32>().unwrap();
        sum + a * b
    });
    println!("{}", mul_result);

    let do_mul_pattern = Regex::new(r"((?<cmd>do|don't)\(\))|(mul\((?<a>[0-9]+),(?<b>[0-9]+)\))").unwrap();
    let do_mul_instructions = input_file.lines()
        .flat_map(|line| do_mul_pattern.captures_iter(line).collect::<Vec<Captures>>())
        .collect::<Vec<Captures>>();

    let mut do_mul = true;
    let do_mul_result = do_mul_instructions.iter().fold(0, |sum, capture| {
        match capture.name("cmd") {
            Some(cmd) => {
                do_mul = cmd.as_str() == "do";
                sum
            },
            None => {
                if do_mul {
                    let a = capture.name("a").unwrap().as_str().parse::<u32>().unwrap();
                    let b = capture.name("b").unwrap().as_str().parse::<u32>().unwrap();
                    sum + a * b
                } else {
                    sum
                }
            }
        }
    });
    println!("{}", do_mul_result);
}
