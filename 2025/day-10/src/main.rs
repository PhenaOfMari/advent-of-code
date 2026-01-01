use std::collections::{HashMap, VecDeque};
use std::fs;

struct Machine {
    indicator: usize,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
    memo: HashMap<usize, Vec<Vec<usize>>>
}

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut machines = Vec::new();
    for line in input_file.lines() {
        let mut indicator = 0;
        let mut buttons = Vec::new();
        let mut joltage = Vec::new();
        let mut tokens = line.split_whitespace();
        while let Some(token) = tokens.next() {
            let trimmed = &token[1..token.len() - 1];
            if token.starts_with("(") {
                let mut wires = Vec::new();
                let mut nums = trimmed.split(",");
                while let Some(num) = nums.next() {
                    wires.push(num.parse::<usize>().unwrap());
                }
                buttons.push(wires);
            } else if token.starts_with("[") {
                for (idx, char) in trimmed.chars().enumerate() {
                    if char == '#' {
                        indicator |= 1 << idx;
                    }
                }
            } else if token.starts_with("{") {
                let mut nums = trimmed.split(",");
                while let Some(num) = nums.next() {
                    joltage.push(num.parse::<usize>().unwrap());
                }
            }
        }
        machines.push(Machine {indicator, buttons, joltage, memo: HashMap::new()});
    }

    let mut presses1 = 0;
    let mut presses2 = 0;
    for m in machines.iter_mut() {
        presses1 += get_options(m, m.indicator)[0].len();
        presses2 += joltage_search(m);
    }
    println!("{}", presses1);
    println!("{}", presses2);
}

fn get_options(machine: &mut Machine, goal: usize) -> Vec<Vec<usize>> {
    if let Some(results) = machine.memo.get(&goal) {
        results.clone()
    } else {
        let mut results = Vec::new();
        let buttons = &machine.buttons;
        for i in 0..2usize.pow(buttons.len() as u32) {
            let mut pressed = Vec::new();
            for j in 0..buttons.len() {
                if (1 << j) & i > 0 {
                    pressed.push(j);
                }
            }
            let mut lights = 0;
            for &button_id in pressed.iter() {
                for &k in buttons[button_id].iter() {
                    lights ^= 1 << k;
                }
            }
            if lights == goal {
                results.push(pressed);
            }
        }
        results.sort_by(|v1, v2| v1.len().cmp(&v2.len()));
        machine.memo.insert(goal, results.clone());
        results
    }
}

fn joltage_search(machine: &mut Machine) -> usize {
    let mut stack = VecDeque::new();
    stack.push_back((machine.joltage.clone(), 0, 1));
    let mut min_presses = usize::MAX;
    while let Some((target, total_presses, multiplier)) = stack.pop_front() {
        let goal = target.iter().enumerate().fold(0, |p, (idx, v)| {
            if v % 2 != 0 {
                p | (1 << idx)
            } else {
                p
            }
        });
        let options = get_options(machine, goal);
        'option: for option in options {
            let mut next_target = target.clone();
            let next_presses = total_presses + option.len() * multiplier;
            for button_id in option {
                let buttons = &machine.buttons[button_id];
                for &id in buttons.iter() {
                    let result = next_target[id].checked_sub(1);
                    if let Some(result) = result {
                        next_target[id] = result;
                    } else {
                        continue 'option;
                    }
                }
            }
            if next_target.iter().all(|&x| x == 0) {
                if next_presses < min_presses {
                    min_presses = next_presses;
                }
            } else {
                for i in 0..next_target.len() {
                    next_target[i] /= 2;
                }
                stack.push_back((next_target, next_presses, multiplier * 2));
            }
        }
    }
    min_presses
}
