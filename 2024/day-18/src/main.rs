use std::collections::{HashMap, HashSet};
use std::fs;
use utils::cartesian::{Cartesian, CARDINALS};
use utils::grid::Grid;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let bytes = input_file.lines()
        .map(|line| {
            let mut tokens = line.split(',');
            let x = tokens.next().unwrap().parse::<i32>().unwrap();
            let y = tokens.next().unwrap().parse::<i32>().unwrap();
            Cartesian::new(x, y)
        })
        .collect::<Vec<Cartesian>>();
    let start = Cartesian::new(0, 0);
    let end = Cartesian::new(70, 70);

    let mut memory = Grid::new(71, 71, '.');
    for i in 0..1024 {
        memory.set(bytes[i], '#')
    }
    let mut distances = HashMap::new();
    traverse(&memory, start, 0, &mut distances);
    println!("{}", distances.get(&end).unwrap());

    let mut last_drop = bytes[1023];
    for i in 1024..bytes.len() {
        last_drop = bytes[i];
        memory.set(last_drop, '#');
        let mut visited = HashSet::new();
        if !check_for_path(&memory, start, end, &mut visited) {
            break;
        }
    }
    println!("{}", last_drop);
}

fn traverse(memory: &Grid<char>, here: Cartesian, step: u16, distances: &mut HashMap<Cartesian, u16>) {
    distances.insert(here, step);
    for direction in CARDINALS {
        let next = here + direction;
        if memory.get(next).is_some_and(|c| c != '#')
            && distances.get(&next).is_none_or(|&d| step + 1 < d) {
            traverse(memory, next, step + 1, distances);
        }
    }
}

fn check_for_path(memory: &Grid<char>, here: Cartesian, target: Cartesian, visited: &mut HashSet<Cartesian>) -> bool {
    if here == target {
        return true
    }
    visited.insert(here);
    CARDINALS.iter().any(|&direction| -> bool {
        let next = here + direction;
        memory.get(next).is_some_and(|c| c != '#')
            && !visited.contains(&next)
            && check_for_path(memory, next, target, visited)
    })
}