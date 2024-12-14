use std::fs;
use utils::cartesian::{Cartesian, DIRECTIONS};
use utils::grid::Grid;

const ROOM_SIZE: Cartesian = Cartesian::new(101, 103);

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let robots = input_file.lines()
        .map(|line| {
            let mut tokens = line.split_whitespace();
            let mut p_tokens = tokens.next().unwrap()
                .strip_prefix("p=").unwrap().split(',');
            let mut v_tokens = tokens.next().unwrap()
                .strip_prefix("v=").unwrap().split(',');
            let position_x = p_tokens.next().unwrap().parse().unwrap();
            let position_y = p_tokens.next().unwrap().parse().unwrap();
            let velocity_x = v_tokens.next().unwrap().parse().unwrap();
            let velocity_y = v_tokens.next().unwrap().parse().unwrap();
            (Cartesian::new(position_x, position_y), Cartesian::new(velocity_x, velocity_y))
        })
        .collect::<Vec<(Cartesian, Cartesian)>>();

    let mut robots_100s = robots.clone();
    simulate(&mut robots_100s, 100);
    let safety = calc_safety(&robots_100s);
    println!("{}", safety);

    let mut robots_egg = robots.clone();
    for i in 0..10000 {
        if check_for_tree(&robots_egg) {
            println!("{}", i);
            break;
        }
        simulate(&mut robots_egg, 1);
    }
    let mut map = Grid::new(ROOM_SIZE.x as usize, ROOM_SIZE.y as usize, ' ');
    for robot in robots_egg.iter() {
        map.set(robot.0, 'X');
    }
    for j in 0..map.width() {
        for i in 0..map.height() {
            let here = Cartesian::new(i as i32, j as i32);
            print!("{}", map.get(here).unwrap());
        }
        println!();
    }
}

fn simulate(robots: &mut Vec<(Cartesian, Cartesian)>, steps: i32) {
    for robot in robots {
        robot.0 = (robot.0 + robot.1 * steps) % ROOM_SIZE;
        if robot.0.x < 0 {
            robot.0.x += ROOM_SIZE.x;
        }
        if robot.0.y < 0 {
            robot.0.y += ROOM_SIZE.y;
        }
    }
}

fn calc_safety(robots: &Vec<(Cartesian, Cartesian)>) -> u32 {
    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;
    for robot in robots.iter() {
        let half_width = ROOM_SIZE.x / 2;
        let half_height = ROOM_SIZE.y / 2;
        if robot.0.x < half_width {
            if robot.0.y < half_height {
                tl += 1;
            }
            if robot.0.y >= ROOM_SIZE.y - half_height {
                tr += 1;
            }
        }
        if robot.0.x >= ROOM_SIZE.x - half_width {
            if robot.0.y < half_height {
                bl += 1;
            }
            if robot.0.y >= ROOM_SIZE.y - half_height {
                br += 1;
            }
        }
    }
    tl * tr * bl * br
}

fn check_for_tree(robots: &Vec<(Cartesian, Cartesian)>) -> bool {
    let mut map = Grid::new(ROOM_SIZE.x as usize, ROOM_SIZE.y as usize, ' ');
    for robot in robots.iter() {
        map.set(robot.0, 'X');
    }
    robots.iter().any(|robot| {
        DIRECTIONS.iter().all(|&dir| {
            map.get(robot.0 + dir).is_some_and(|c| c == 'X')
        })
    })
}
