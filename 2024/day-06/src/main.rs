use std::collections::HashSet;
use std::fs;
use utils::cartesian::Cartesian;
use utils::grid::Grid;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut start = Cartesian::new(0, 0);
    let mut start_found = false;
    let mut layout = input_file.lines()
        .map(|line| {
            if !start_found {
                let caret_index = line.find('^');
                match caret_index {
                    Some(index) => {
                        start.y = index as i32;
                        start_found = true;
                    }
                    None => {
                        start.x += 1;
                    }
                }
            }
            line.chars().collect()
        })
        .collect::<Grid<char>>();

    let mut position = start;
    let mut heading = Cartesian::UP;
    loop {
        layout.set(position, 'X');
        let tentative = position + heading;
        match layout.get(tentative) {
            Some('#') => {
                heading = heading.quarter_turn(false);
            },
            Some(_) => {
                position = tentative;
            },
            None => break
        }
    }
    let positions_occupied = layout.data().iter()
        .fold(0, |sum, row| sum + row.iter().filter(|c| **c == 'X').count());
    println!("{}", positions_occupied);

    let mut position = start;
    let mut heading = Cartesian::UP;
    loop {
        let tentative = position + heading;
        let spot = layout.get(tentative);
        if spot == None {
            break;
        } else if spot == Some('#') {
            heading = heading.quarter_turn(false);
        } else if tentative == start || spot == Some('O') {
            position = tentative;
        } else {
            let mut previous_turns = HashSet::new();
            let mut position2 = start;
            let mut heading2 = Cartesian::UP;
            loop {
                let tentative2 = position2 + heading2;
                let spot2 = layout.get(tentative2);
                if spot2 == None {
                    break;
                } else if tentative2 == tentative || spot2 == Some('#') {
                    if !previous_turns.insert((position2, heading2)) {
                        layout.set(tentative, 'O');
                        break;
                    }
                    heading2 = heading2.quarter_turn(false);
                } else {
                    position2 = tentative2;
                }
            }
            position = tentative;
        }
    }
    let obstacle_spots = layout.data().iter()
        .fold(0, |sum, row| sum + row.iter().filter(|c| **c == 'O').count());
    println!("{}", obstacle_spots);
}
