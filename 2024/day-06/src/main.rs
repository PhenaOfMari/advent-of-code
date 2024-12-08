use std::f64::consts::FRAC_PI_2;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut start = (0, 0);
    let mut start_found = false;
    let mut layout = input_file.lines()
        .map(|line| {
            if !start_found {
                let caret_index = line.find('^');
                match caret_index {
                    Some(index) => {
                        start.1 = index as i16;
                        start_found = true;
                    }
                    None => {
                        start.0 = start.0 + 1;
                    }
                }
            }
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let map_height = layout.len() as i16;
    let map_width = layout[0].len() as i16;
    let out_of_bounds = |position: (i16, i16)| {
        position.0 < 0 || position.0 >= map_height || position.1 < 0 || position.1 >= map_width
    };

    let mut position = start;
    let mut heading = (-1, 0);
    loop {
        layout[position.0 as usize][position.1 as usize] = 'X';
        let tentative = (position.0 + heading.0, position.1 + heading.1);
        if out_of_bounds(tentative) {
            break;
        }
        if layout[tentative.0 as usize][tentative.1 as usize] == '#' {
            heading = rotate_heading(heading);
        } else {
            position = tentative;
        }
    }
    let positions_occupied = layout.iter()
        .fold(0, |sum, row| sum + row.iter().filter(|c| **c == 'X').count());
    println!("{}", positions_occupied);

    let mut position = start;
    let mut heading = (-1, 0);
    loop {
        let tentative = (position.0 + heading.0, position.1 + heading.1);
        if out_of_bounds(tentative) {
            break;
        }
        if layout[tentative.0 as usize][tentative.1 as usize] == '#' {
            heading = rotate_heading(heading);
        } else if tentative == start || layout[tentative.0 as usize][tentative.1 as usize] == 'O' {
            position = tentative;
        } else {
            let mut previous_turns = vec![];
            let mut position2 = start;
            let mut heading2 = (-1, 0);
            loop {
                let tentative2 = (position2.0 + heading2.0, position2.1 + heading2.1);
                if out_of_bounds(tentative2) {
                    break;
                }
                if tentative2 == tentative || layout[tentative2.0 as usize][tentative2.1 as usize] == '#' {
                    if previous_turns.contains(&(position2, heading2)) {
                        layout[tentative.0 as usize][tentative.1 as usize] = 'O';
                        break;
                    }
                    previous_turns.push((position2, heading2));
                    heading2 = rotate_heading(heading2);
                } else {
                    position2 = tentative2;
                }
            }
            position = tentative;
        }
    }
    let obstacle_spots = layout.iter()
        .fold(0, |sum, row| sum + row.iter().filter(|c| **c == 'O').count());
    println!("{}", obstacle_spots);
}

fn rotate_heading(heading: (i16, i16)) -> (i16, i16) {
    let heading = (heading.0 as f64).atan2(heading.1 as f64) + FRAC_PI_2;
    let (sin, cos) = heading.sin_cos();
    (sin.round() as i16, cos.round() as i16)
}
