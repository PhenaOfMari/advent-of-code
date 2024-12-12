use std::collections::HashSet;
use std::fs;
use utils::cartesian::{Cartesian, CARDINALS};
use utils::grid::Grid;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let map = input_file.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect::<Grid<u8>>();

    let mut score = 0;
    for i in 0..map.height() {
        for j in 0..map.width() {
            let here = Cartesian::new(i as i32, j as i32);
            if map.get(here) == Some(0) {
                score += score_trail(&map, here, None, &mut HashSet::new());
            }
        }
    }
    println!("{}", score);

    let mut rating = 0;
    for i in 0..map.height() {
        for j in 0..map.width() {
            let here = Cartesian::new(i as i32, j as i32);
            if map.get(here) == Some(0) {
                rating += rate_trail(&map, here, None);
            }
        }
    }
    println!("{}", rating);
}

fn score_trail(map: &Grid<u8>, here: Cartesian, previous: Option<u8>, path: &mut HashSet<Cartesian>) -> u16 {
    match map.get(here) {
        None => 0,
        Some(i) => {
            if previous.is_some_and(|p| i == p + 1 && i == 9) {
                if path.insert(here) {
                    1
                } else {
                    0
                }
            } else if previous.is_none_or(|p| i == p + 1) {
                if path.insert(here) {
                    CARDINALS.iter().fold(0, |sum, &direction| {
                        sum + score_trail(map, here + direction, Some(i), path)
                    })
                } else {
                    0
                }
            } else {
                0
            }
        }
    }
}

fn rate_trail(map: &Grid<u8>, here: Cartesian, previous: Option<u8>) -> u16 {
    match map.get(here) {
        None => 0,
        Some(i) => {
            if previous.is_some_and(|p| i == p + 1 && i == 9) {
                1
            } else if previous.is_none_or(|p| i == p + 1) {
                CARDINALS.iter().fold(0, |sum, &direction| {
                    sum + rate_trail(map, here + direction, Some(i))
                })
            } else {
                0
            }
        }
    }
}
