use std::collections::HashMap;
use std::fs;
use utils::cartesian::{Cartesian, CARDINALS};

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let codes = input_file.lines().collect::<Vec<&str>>();
    let numpad = HashMap::from([
        ('X', Cartesian::new(3, 0)),
        ('0', Cartesian::new(3, 1)),
        ('A', Cartesian::new(3, 2)),
        ('1', Cartesian::new(2, 0)),
        ('2', Cartesian::new(2, 1)),
        ('3', Cartesian::new(2, 2)),
        ('4', Cartesian::new(1, 0)),
        ('5', Cartesian::new(1, 1)),
        ('6', Cartesian::new(1, 2)),
        ('7', Cartesian::new(0, 0)),
        ('8', Cartesian::new(0, 1)),
        ('9', Cartesian::new(0, 2))
    ]);
    let keypad = HashMap::from([
        ('X', Cartesian::new(0, 0)),
        ('^', Cartesian::new(0, 1)),
        ('A', Cartesian::new(0, 2)),
        ('<', Cartesian::new(1, 0)),
        ('v', Cartesian::new(1, 1)),
        ('>', Cartesian::new(1, 2))
    ]);
    let mut memo = HashMap::new();

    let complexity = codes.iter().fold(0, |sum, code| {
        let len = calc_length(&numpad, &keypad, code, 3, true, &mut memo);
        let num = code[0..code.len() - 1].parse::<usize>().unwrap();
        sum + len * num
    });
    println!("{}", complexity);

    let complexity2 = codes.iter().fold(0, |sum, code| {
        let len = calc_length(&numpad, &keypad, code, 26, true, &mut memo);
        let num = code[0..code.len() - 1].parse::<usize>().unwrap();
        sum + len * num
    });
    println!("{}", complexity2);
}

fn calc_length(numpad: &HashMap<char, Cartesian>, keypad: &HashMap<char, Cartesian>,
               sequence: &str, mutations: u8, num: bool,
               memo: &mut HashMap<(String, u8), usize>) -> usize {
    if mutations == 0 {
        sequence.len()
    } else {
        let key = (sequence.to_string(), mutations);
        if memo.contains_key(&key) {
            return memo[&key]
        }
        let pad = if num {numpad} else {keypad};
        let avoid = *pad.get(&'X').unwrap();
        let mut pos = *pad.get(&'A').unwrap();
        let length = sequence.chars()
            .map(|c| {
                let target = *pad.get(&c).unwrap();
                let cheapest = paths(pos, target, avoid).iter()
                    .map(|option| calc_length(numpad, keypad, option, mutations - 1, false, memo))
                    .fold(usize::MAX, |sum, item| {
                        sum.min(item)
                    });
                pos = target;
                cheapest
            })
            .fold(0, |sum, item| sum + item);
        memo.insert(key, length);
        length
    }
}

fn paths(start: Cartesian, end: Cartesian, avoid: Cartesian) -> Vec<String> {
    if start == end {
        return vec!["A".parse().unwrap()]
    }
    let distance = start.distance(end);
    let map = HashMap::from([(Cartesian::UP, '^'), (Cartesian::DOWN, 'v'), (Cartesian::LEFT, '<'), (Cartesian::RIGHT, '>')]);
    let mut output = Vec::new();
    for direction in CARDINALS {
        let next = start + direction;
        if next != avoid && next.distance(end) < distance {
            let results = paths(next, end, avoid);
            for result in results {
                let mut str = map.get(&direction).unwrap().to_string();
                str.push_str(&result);
                output.push(str);
            }
        }
    }
    output
}
