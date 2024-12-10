use std::collections::{HashMap, HashSet};
use std::fs;
use utils::cartesian::Cartesian;
use utils::grid::Grid;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let map = input_file.lines()
        .map(|line| line.chars().collect())
        .collect::<Grid<char>>();
    let mut frequency_map = HashMap::new();
    for (i, row) in map.data().iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != '.' {
                let set = frequency_map.entry(c).or_insert(HashSet::new());
                set.insert(Cartesian::new(i as i32, j as i32));
            }
        }
    }

    let mut anti_nodes = HashSet::new();
    frequency_map.iter().for_each(|(_, locations)| {
        for &a in locations.iter() {
            for &b in locations.iter() {
                if a == b {
                    continue;
                }
                let difference = b - a;
                let first = a - difference;
                if map.is_in_bounds(first) {
                    anti_nodes.insert(first);
                }
                let second = b + difference;
                if map.is_in_bounds(second) {
                    anti_nodes.insert(second);
                }
            }
        }
    });
    println!("{}", anti_nodes.len());

    let mut harmonics = HashSet::new();
    frequency_map.iter().for_each(|(_, locations)| {
        for &a in locations.iter() {
            for &b in locations.iter() {
                if a == b {
                    continue;
                }
                let difference = b - a;
                let mut previous = a;
                while map.is_in_bounds(previous) {
                    harmonics.insert(previous);
                    previous -= difference;
                }
                let mut next = b;
                while map.is_in_bounds(next) {
                    harmonics.insert(next);
                    next += difference;
                }
            }
        }
    });
    println!("{}", harmonics.len());
}
