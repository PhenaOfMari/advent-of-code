use std::fs;
use utils::cartesian::Cartesian;
use utils::grid::Grid;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let puzzle = input_file.lines()
        .map(|line| line.chars().collect())
        .collect::<Grid<char>>();
    let height = puzzle.height();
    let width = puzzle.width();

    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut xmas_count = 0;
    for i in 0..height {
        for j in 0..width {
            for dir in directions.iter() {
                let here = Cartesian::new(i as i32, j as i32);
                let step = Cartesian::new(dir.0, dir.1);
                if puzzle.get(here) == Some('X')
                    && puzzle.get(here + step) == Some('M')
                    && puzzle.get(here + step * 2) == Some('A')
                    && puzzle.get(here + step * 3) == Some('S') {
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
            let step1 = Cartesian::new(1, 1);
            let step2 = Cartesian::new(-1, 1);
            let check_m_s = |step| {
                puzzle.get(here - step) == Some('M')
                    && puzzle.get(here + step) == Some('S')
                || (puzzle.get(here + step) == Some('M')
                    && puzzle.get(here - step) == Some('S'))
            };
            if puzzle.get(here) == Some('A') && check_m_s(step1) && check_m_s(step2) {
                x_mas_count += 1;
            }
        }
    }
    println!("{}", x_mas_count);
}
