use std::collections::HashSet;
use std::f64::consts::FRAC_PI_2;
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
    let mut heading = Cartesian::new(-1, 0);
    loop {
        layout.set_coordinate(position, 'X');
        let tentative = position + heading;
        match layout.get_coordinate(tentative) {
            Some('#') => {
                heading = rotate_heading(heading);
            },
            Some(_) => {
                position = tentative;
            },
            None => break
        }
    }
    let positions_occupied = layout.get_data().iter()
        .fold(0, |sum, row| sum + row.iter().filter(|c| **c == 'X').count());
    println!("{}", positions_occupied);

    let mut position = start;
    let mut heading = Cartesian::new(-1, 0);
    loop {
        let tentative = position + heading;
        let spot = layout.get_coordinate(tentative);
        if spot == None {
            break;
        } else if spot == Some('#') {
            heading = rotate_heading(heading);
        } else if tentative == start || spot == Some('O') {
            position = tentative;
        } else {
            let mut previous_turns = HashSet::new();
            let mut position2 = start;
            let mut heading2 = Cartesian::new(-1, 0);
            loop {
                let tentative2 = position2 + heading2;
                let spot2 = layout.get_coordinate(tentative2);
                if spot2 == None {
                    break;
                } else if tentative2 == tentative || spot2 == Some('#') {
                    if !previous_turns.insert((position2, heading2)) {
                        layout.set_coordinate(tentative, 'O');
                        break;
                    }
                    heading2 = rotate_heading(heading2);
                } else {
                    position2 = tentative2;
                }
            }
            position = tentative;
        }
    }
    let obstacle_spots = layout.get_data().iter()
        .fold(0, |sum, row| sum + row.iter().filter(|c| **c == 'O').count());
    println!("{}", obstacle_spots);
}

fn rotate_heading(heading: Cartesian) -> Cartesian {
    let heading = (heading.x as f64).atan2(heading.y as f64) + FRAC_PI_2;
    let (sin, cos) = heading.sin_cos();
    Cartesian::new(sin.round() as i32, cos.round() as i32)
}
