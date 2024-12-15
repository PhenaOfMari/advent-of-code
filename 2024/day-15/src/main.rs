use std::fs;
use utils::cartesian::Cartesian;
use utils::grid::Grid;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut start = Cartesian::new(0, 0);
    let mut start_found = false;
    let mut map_data = Vec::new();
    let mut commands = Vec::new();
    for line in input_file.lines() {
        if line.starts_with("#") {
            let at_index = line.find('@');
            match at_index {
                Some(index) => {
                    start.y = index as i32;
                    start_found = true;
                }
                None => {
                    if !start_found {
                        start.x += 1;
                    }
                }
            }
            map_data.push(line.chars().collect::<Vec<char>>());
        } else {
            line.chars().for_each(|c| {
                match c {
                    '^' => commands.push(Cartesian::UP),
                    '>' => commands.push(Cartesian::RIGHT),
                    'v' => commands.push(Cartesian::DOWN),
                    '<' => commands.push(Cartesian::LEFT),
                    _ => ()
                }
            })
        }
    }

    let mut map = map_data.clone().into_iter().collect::<Grid<char>>();
    let mut robot_location = start;
    for &command in commands.iter() {
        if can_push(&mut map, robot_location, command) {
            push(&mut map, robot_location, command);
            robot_location += command;
        }
    }
    let mut sum = 0;
    for i in 1..map.height() - 1 {
        for j in 1..map.width() - 1 {
            if map.data()[i][j] == 'O' {
                sum += 100 * i + j;
            }
        }
    }
    println!("{}", sum);

    let mut wide_map = map_data.iter().map(|row| {
        let mut new_row = Vec::new();
        for &c in row {
            if c == 'O' {
                new_row.push('[');
                new_row.push(']');
            } else if c == '@' {
                new_row.push(c);
                new_row.push('.');
            } else {
                new_row.push(c);
                new_row.push(c);
            }
        }
        new_row
    }).collect::<Grid<char>>();
    let mut wide_robot_location = Cartesian::new(start.x, start.y * 2);
    for &command in commands.iter() {
        if can_push(&mut wide_map, wide_robot_location, command) {
            push(&mut wide_map, wide_robot_location, command);
            wide_robot_location += command;
        }
    }
    let mut wide_sum = 0;
    for i in 1..wide_map.height() - 1 {
        for j in 2..wide_map.width() - 3 {
            if wide_map.data()[i][j] == '[' {
                wide_sum += 100 * i + j;
            }
        }
    }
    println!("{}", wide_sum);
}

fn can_push(map: &mut Grid<char>, here: Cartesian, direction: Cartesian) -> bool {
    let next = here + direction;
    match map.get(next) {
        Some(c) => {
            match c {
                '#' => false,
                'O' => can_push(map, next, direction),
                '[' => direction == Cartesian::LEFT || (can_push(map, next, direction) && can_push(map, next + Cartesian::RIGHT, direction)),
                ']' => direction == Cartesian::RIGHT || (can_push(map, next + Cartesian::LEFT, direction) && can_push(map, next, direction)),
                _ => true
            }
        },
        None => false
    }
}

fn push(map: &mut Grid<char>, here: Cartesian, direction: Cartesian) {
    let next = here + direction;
    let current = map.get(here).unwrap();
    let lookahead = map.get(next).unwrap();
    if lookahead == 'O' {
        push(map, next, direction);
    } else if direction == Cartesian::UP || direction == Cartesian::DOWN {
        if lookahead == '[' {
            push(map, next, direction);
            if lookahead != current {
                push(map, next + Cartesian::RIGHT, direction);
            }
        } else if lookahead == ']' {
            if lookahead != current {
                push(map, next + Cartesian::LEFT, direction);
            }
            push(map, next, direction);
        }
    } else if lookahead == '[' || lookahead == ']' {
        push(map, next, direction);
    }
    map.set(next, current);
    map.set(here, '.');
}
