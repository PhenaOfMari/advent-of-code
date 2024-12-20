use std::fs;
use utils::cartesian::{Cartesian, CARDINALS};
use utils::grid::Grid;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut start = Cartesian::new(0, 0);
    let mut start_found = false;
    let mut end = Cartesian::new(0, 0);
    let mut end_found = false;
    let course = input_file.lines()
        .map(|line| {
            if !start_found {
                let s_index = line.find('S');
                match s_index {
                    Some(index) => {
                        start.y = index as i32;
                        start_found = true;
                    }
                    None => {
                        start.x += 1;
                    }
                }
            }
            if !end_found {
                let e_index = line.find('E');
                match e_index {
                    Some(index) => {
                        end.y = index as i32;
                        end_found = true;
                    }
                    None => {
                        end.x += 1;
                    }
                }
            }
            line.chars().collect()
        })
        .collect::<Grid<char>>();
    let mut timed_course = Grid::new(course.height(), course.width(), u16::MAX);
    time_course(&course, start, 0, &mut timed_course);

    let cheat_for_2 = count_cheats(&timed_course, 100, 2);
    println!("{}", cheat_for_2);

    let cheat_for_20 = count_cheats(&timed_course, 100, 20);
    println!("{}", cheat_for_20);
}

fn time_course(course: &Grid<char>, here: Cartesian, time: u16, timed_course: &mut Grid<u16>) {
    timed_course.set(here, time);
    for direction in CARDINALS {
        let next = here + direction;
        if course.get(next).is_some_and(|c| c != '#') && timed_course.get(next).unwrap() == u16::MAX {
            time_course(course, next, time + 1, timed_course);
        }
    }
}

fn count_cheats(timed_course: &Grid<u16>, time_to_save: u16, max_distance: i32) -> u32 {
    let height = timed_course.height();
    let width = timed_course.width();
    let mut count = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let here = Cartesian::new(i as i32, j as i32);
            let now = timed_course.get(here).unwrap();
            if now != u16::MAX {
                let most_up = (here.x - max_distance).max(1) as usize;
                let most_down = (here.x + max_distance).min(height as i32 - 2) as usize;
                let most_left = (here.y - max_distance).max(1) as usize;
                let most_right = (here.y + max_distance).min(width as i32 - 2) as usize;
                for k in most_up..most_down + 1 {
                    for l in most_left..most_right + 1 {
                        let there = Cartesian::new(k as i32, l as i32);
                        let distance = here.distance(there);
                        if distance > max_distance {
                            continue;
                        }
                        let with = now + distance as u16;
                        let then = timed_course.get(there).unwrap();
                        if then != u16::MAX && then > with && then - with >= time_to_save {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}
