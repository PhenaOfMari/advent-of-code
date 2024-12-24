use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

const INPUT_LEN: usize = 45;
const OUTPUT_LEN: usize = INPUT_LEN + 1;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut initial_wires = [u8::MAX; 17576];
    let mut instructions = VecDeque::new();
    for line in input_file.lines() {
        if line.contains(":") {
            initial_wires[get_index(&line[0..3])] = line[5..].parse::<u8>().unwrap();
        } else if line.contains("->") {
            let mut split = line.split_whitespace();
            let a = split.next().unwrap();
            let op = split.next().unwrap();
            let b = split.next().unwrap();
            split.next();
            let c = split.next().unwrap();
            instructions.push_back((op, a, b, c));
        }
    }

    let mut wires = initial_wires.clone();
    compute(&mut wires, &instructions);
    let z = read(&wires, 'z', OUTPUT_LEN);
    println!("{}", z);

    // x[i] XOR y[i] -> half_sum[i]
    // x[i] AND y[i] -> half_carry[i]
    // half_sum[i] XOR carry[i-1] -> z[i]
    // half_sum[i] AND carry[i-1] -> tmp[i]
    // tmp[i] OR half_carry[i] -> carry[i]
    let mut instruction_map = HashMap::new();
    for (op, a, b, c) in instructions.iter() {
        instruction_map.insert((*a, *op), (*b, *c));
        instruction_map.insert((*b, *op), (*a, *c));
    }
    let mut swap_set = HashSet::new();
    let mut last_carry = instruction_map.get(&("x00", "AND")).unwrap().1;
    for i in 1..INPUT_LEN {
        let xi = format!("x{:0>2}", i);
        let zi = format!("z{:0>2}", i);
        let (_, half_sum) = *instruction_map.get(&(xi.as_str(), "XOR")).unwrap();
        let (_, half_carry) = *instruction_map.get(&(xi.as_str(), "AND")).unwrap();
        let z_inst = instruction_map.get(&(last_carry, "XOR"));
        let z_inst = if z_inst.is_some() {
            let inst = *z_inst.unwrap();
            if inst.0 != half_sum {
                swap_set.insert(half_sum.to_string());
                swap_set.insert(inst.0.to_string());
            }
            inst
        } else {
            let inst = *instruction_map.get(&(half_sum, "XOR")).unwrap();
            if inst.0 != last_carry {
                swap_set.insert(last_carry.to_string());
                swap_set.insert(inst.0.to_string());
            }
            inst
        };
        if z_inst.1 != zi.as_str() {
            swap_set.insert(zi);
            swap_set.insert(z_inst.1.to_string());
        }
        let tmp_inst = instruction_map.get(&(last_carry, "AND"));
        let (_, tmp) = if tmp_inst.is_some() {
            let inst = *tmp_inst.unwrap();
            if inst.0 != half_sum {
                swap_set.insert(half_sum.to_string());
                swap_set.insert(inst.0.to_string());
            }
            inst
        } else {
            let inst = *instruction_map.get(&(half_sum, "AND")).unwrap();
            if inst.0 != last_carry {
                swap_set.insert(last_carry.to_string());
                swap_set.insert(inst.0.to_string());
            }
            inst
        };
        let c_inst = instruction_map.get(&(half_carry, "OR"));
        let (_, carry) = if c_inst.is_some() {
            let inst = *c_inst.unwrap();
            if inst.0 != tmp {
                swap_set.insert(tmp.to_string());
                swap_set.insert(inst.0.to_string());
            }
            inst
        } else {
            let inst = *instruction_map.get(&(tmp, "OR")).unwrap();
            if inst.0 != half_carry {
                swap_set.insert(half_carry.to_string());
                swap_set.insert(inst.0.to_string());
            }
            inst
        };
        last_carry = carry;
    }
    let mut swap_list = swap_set.iter().cloned().collect::<Vec<String>>();
    swap_list.sort();
    println!("{}", swap_list.join(","));
}

fn compute<'a>(workspace: &mut [u8], instructions: &VecDeque<(&'a str, &'a str, &'a str, &'a str)>) {
    let mut remaining = instructions.clone();
    let mut impossible = 0;
    while !remaining.is_empty() && impossible < remaining.len() {
        let instruction = remaining.pop_front().unwrap();
        let input1 = workspace[get_index(instruction.1)];
        let input2 = workspace[get_index(instruction.2)];
        if input1 != u8::MAX && input2 != u8::MAX {
            impossible = 0;
            match instruction.0 {
                "AND" => workspace[get_index(instruction.3)] = input1 & input2,
                "OR" => workspace[get_index(instruction.3)] = input1 | input2,
                "XOR" => workspace[get_index(instruction.3)] = input1 ^ input2,
                _ => unreachable!()
            }
        } else {
            impossible += 1;
            remaining.push_back(instruction);
        }
    }
}

fn get_index(input: &str) -> usize {
    let first = input.chars().next().unwrap() as usize - 97;
    if first > 22 {
        first * 676 + input[1..].parse::<usize>().unwrap()
    } else {
        input.chars().fold(0, |hash, c| hash * 26 + (c as usize - 97))
    }
}

fn read(wires: &[u8], starting_char: char, len: usize) -> u64 {
    let start = (starting_char as usize - 97) * 676;
    let mut value = 0;
    for &bit in wires[start..start+len].iter().rev() {
        value = (value << 1) + bit as u64;
    }
    value
}
