use std::fs;
use utils::cartesian::{Cartesian, CARDINALS};
use utils::grid::Grid;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut maze = input_file.lines()
        .map(|line| line.chars().collect())
        .collect::<Grid<char>>();
    let start = Cartesian::new((maze.height() - 2) as i32, 1);
    let end = Cartesian::new(1, (maze.width() - 2) as i32);

    let mut scored_maze = maze.data().iter()
        .map(|row| row.iter()
            .map(|&c| if c == '#' {
                -1
            } else {
                i32::MAX
            }).collect())
        .collect::<Grid<i32>>();
    score_maze(&mut scored_maze, start, Cartesian::RIGHT, end, 0);
    let best_score = scored_maze.get(end).unwrap();
    println!("{}", best_score);

    mark_seating(&mut maze, &scored_maze, end, Cartesian::LEFT);
    mark_seating(&mut maze, &scored_maze, end, Cartesian::DOWN);
    let mut seat_count = 0;
    for i in 1..maze.height() -1 {
        for j in 1..maze.width() - 1 {
            if maze.get(Cartesian::new(i as i32, j as i32)) == Some('O') {
                seat_count += 1;
            }
        }
    }
    println!("{}", seat_count);
}

fn score_maze(maze: &mut Grid<i32>, here: Cartesian, direction: Cartesian, target:Cartesian, current_score: i32) {
    maze.set(here, current_score);
    if here == target {
        return
    }
    let left = direction.quarter_turn(true);
    let right = direction.quarter_turn(false);
    let look_ahead = maze.get(here + direction).unwrap();
    let look_left = maze.get(here + left).unwrap();
    let look_right = maze.get(here + right).unwrap();
    if look_left > 0 && current_score + 1001 < look_left {
        score_maze(maze, here + left, left, target, current_score + 1001);
    }
    if look_right > 0 && current_score + 1001 < look_right {
        score_maze(maze, here + right, right, target, current_score + 1001);
    }
    if look_ahead > 0 && current_score + 1 < look_ahead {
        score_maze(maze, here + direction, direction, target, current_score + 1);
    }
}

fn mark_seating(maze: &mut Grid<char>, scored_maze: &Grid<i32>, here: Cartesian, direction: Cartesian) {
    maze.set(here, 'O');
    let score_here = scored_maze.get(here).unwrap();
    for next_direction in CARDINALS {
        let next = here + next_direction;
        let look_ahead = maze.get(next).unwrap();
        let score_next = scored_maze.get(next).unwrap();
        if look_ahead != 'O' && look_ahead != '#'
            && (score_next < score_here || scored_maze.get(here - direction).unwrap() == score_next + 2) {
            mark_seating(maze, scored_maze, next, next_direction);
        }
    }
}
