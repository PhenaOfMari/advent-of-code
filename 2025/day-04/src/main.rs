use std::fs;

const DIRECTIONS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut map = Vec::new();
    for line in input_file.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        map.push(row);
    }

    let mut count1 = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if is_removable(&map, i, j) {
                count1 += 1;
            }
        }
    }
    println!("{}", count1);

    let mut count2 = 0;
    let mut i = 0;
    let mut j = 0;
    while i < map.len() {
        while j < map[i].len() {
            if is_removable(&map, i, j) {
                count2 += 1;
                map[i][j] = '.';
                if i > 0 {
                    i -= 1;
                }
                if j > 0 {
                    j -= 1;
                }
                continue;
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }
    println!("{}", count2);
}

fn is_removable(map: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if map[i][j] == '@' {
        let mut neighbors = 0;
        for dir in DIRECTIONS.iter() {
            let k = i as i32 + dir.0;
            let l = j as i32 + dir.1;
            if k >= 0 && k < map.len() as i32 && l >= 0 && l < map[i].len() as i32 {
                if map[k as usize][l as usize] == '@' {
                    neighbors += 1;
                }
            }
        }
        neighbors < 4
    } else {
        false
    }
}
