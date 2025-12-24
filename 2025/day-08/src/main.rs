use std::cmp::Ordering;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut boxes = Vec::new();
    for line in input_file.lines() {
        let mut tokens = line.split(",");
        let x = tokens.next().unwrap().parse::<usize>().unwrap();
        let y = tokens.next().unwrap().parse::<usize>().unwrap();
        let z = tokens.next().unwrap().parse::<usize>().unwrap();
        boxes.push((x, y, z));
    }

    let num_boxes = boxes.len();
    let mut circuit_ids = (0..num_boxes).collect::<Vec<usize>>();
    let mut circuit_sizes = vec![1usize; num_boxes];
    let mut distances = Vec::new();
    for i in 0..num_boxes - 1 {
        for j in i + 1..num_boxes {
            let (x1, y1, z1) = boxes[i];
            let (x2, y2, z2) = boxes[j];
            let x_diff = x1.max(x2) - x1.min(x2);
            let y_diff = y1.max(y2) - y1.min(y2);
            let z_diff = z1.max(z2) - z1.min(z2);
            let sq = x_diff.pow(2) + y_diff.pow(2) + z_diff.pow(2);
            let distance = (sq as f64).sqrt();
            distances.push((distance, i, j));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

    let mut i = 0;
    let mut connections = 0;
    let mut sizes_after_1k = Vec::new();
    let mut last_x1 = usize::MAX;
    let mut last_x2 = usize::MAX;
    while connections < 999 {
        let (_, a, b) = distances[i];
        let new_id = circuit_ids[a];
        let old_id = circuit_ids[b];
        if new_id != old_id {
            connections += 1;
            if connections == 999 {
                last_x1 = boxes[a].0;
                last_x2 = boxes[b].0;
            }
            let mut num_in_b = 0;
            for j in 0..num_boxes {
                if circuit_ids[j] == old_id {
                    num_in_b += 1;
                    circuit_ids[j] = new_id;
                }
            }
            circuit_sizes[new_id] += num_in_b;
            circuit_sizes[old_id] = 0;
        }
        i += 1;
        if i == 1000 {
            sizes_after_1k = circuit_sizes.clone();
        }
    }

    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    for &size in sizes_after_1k.iter() {
        if size > first {
            third = second;
            second = first;
            first = size;
        } else if size > second {
            third = second;
            second = size;
        } else if size > third {
            third = size;
        }
    }
    println!("{}", first * second * third);
    println!("{}", last_x1 * last_x2);
}
