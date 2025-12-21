use std::collections::HashMap;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut map = Vec::new();
    let mut start = (0usize, usize::MAX);
    let rows = input_file.lines().count();
    for line in input_file.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        if start.1 == usize::MAX {
            let found = chars.iter().enumerate().find(|(_, c)| **c == 'S');
            if let Some((i, _)) = found {
                start.1 = i;
            } else {
                start.0 += 1;
            }
        }
        map.push(chars);
    }

    let mut beam_splits = 0;
    let mut time_splits = 0;
    let mut beams = HashMap::new();
    beams.insert(start, 1);
    while !beams.is_empty() {
        let tmp = beams.clone();
        for (beam, num) in tmp {
            beams.remove(&beam);
            let next_row = beam.0 + 1;
            if next_row < rows {
                let row = &map[next_row];
                if row[beam.1] == '^' {
                    beam_splits += 1;
                    if beam.1 > 0 {
                        merge(&mut beams, (next_row, beam.1 - 1), num);
                    }
                    if beam.1 < row.len() - 1 {
                        merge(&mut beams, (next_row, beam.1 + 1), num);
                    }
                } else {
                    merge(&mut beams, (next_row, beam.1), num);
                }
            } else {
                time_splits += num;
            }
        }
    }
    println!("{}", beam_splits);
    println!("{}", time_splits);
}

fn merge(map: &mut HashMap<(usize, usize), usize>, key: (usize, usize), value: usize) {
    if let Some(old_value) = map.insert(key, value) {
        map.insert(key, old_value + value);
    }
}
