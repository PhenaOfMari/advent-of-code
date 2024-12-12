use std::fs;
use utils::cartesian::{Cartesian, DIRECTIONS};
use utils::grid::Grid;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let puzzle = input_file.lines()
        .map(|line| line.chars().collect())
        .collect::<Grid<char>>();
    let height = puzzle.height();
    let width = puzzle.width();

    let mut xmas_count = 0;
    for i in 0..height {
        for j in 0..width {
            for direction in DIRECTIONS {
                let here = Cartesian::new(i as i32, j as i32);
                if puzzle.get(here) == Some('X')
                    && puzzle.get(here + direction) == Some('M')
                    && puzzle.get(here + direction * 2) == Some('A')
                    && puzzle.get(here + direction * 3) == Some('S') {
                    xmas_count += 1;
                }
            }
        }
    }
    println!("{}", xmas_count);

    let mut x_mas_count = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let here = Cartesian::new(i as i32, j as i32);
            let check_m_s = |step| {
                puzzle.get(here - step) == Some('M')
                    && puzzle.get(here + step) == Some('S')
                || (puzzle.get(here + step) == Some('M')
                    && puzzle.get(here - step) == Some('S'))
            };
            if puzzle.get(here) == Some('A') && check_m_s(Cartesian::UP_RIGHT) && check_m_s(Cartesian::UP_LEFT) {
                x_mas_count += 1;
            }
        }
    }
    println!("{}", x_mas_count);
}
