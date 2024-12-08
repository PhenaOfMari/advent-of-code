use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let puzzle = input_file.lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let height = puzzle.len();
    let width = puzzle[0].len();

    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut xmas_count = 0;
    for i in 0..height {
        for j in 0..width {
            for dir in directions.iter() {
                let i = i as i16;
                let j = j as i16;
                let max_i_diff = i + dir.0 * 3;
                let max_j_diff = j + dir.1 * 3;
                if max_i_diff >= 0 && max_i_diff < height as i16
                    && max_j_diff >= 0 && max_j_diff < width as i16
                    && puzzle[i as usize][j as usize] == 'X'
                    && puzzle[(i + dir.0) as usize][(j + dir.1) as usize] == 'M'
                    && puzzle[(i + dir.0 * 2) as usize][(j + dir.1 * 2) as usize] == 'A'
                    && puzzle[(i + dir.0 * 3) as usize][(j + dir.1 * 3) as usize] == 'S' {
                    xmas_count += 1;
                }
            }
        }
    }
    println!("{}", xmas_count);

    let mut x_mas_count = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if puzzle[i][j] == 'A'
                && ((puzzle[i - 1][j - 1] == 'M' && puzzle[i + 1][j + 1] == 'S')
                    || (puzzle[i + 1][j + 1] == 'M' && puzzle[i - 1][j - 1] == 'S'))
                && ((puzzle[i - 1][j + 1] == 'M' && puzzle[i + 1][j - 1] == 'S')
                    || (puzzle[i + 1][j - 1] == 'M' && puzzle[i - 1][j + 1] == 'S')) {
                x_mas_count += 1;
            }
        }
    }
    println!("{}", x_mas_count);
}
