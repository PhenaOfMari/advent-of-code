use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut coords = Vec::new();
    for line in input_file.lines() {
        let mut tokens = line.split(",");
        let x = tokens.next().unwrap().parse::<usize>().unwrap();
        let y = tokens.next().unwrap().parse::<usize>().unwrap();
        coords.push((x, y));
    }
    let num_coords = coords.len();

    let mut largest = 0;
    let mut contained = 0;
    for i in 0..num_coords - 1 {
        let (x1, y1) = coords[i];
        for j in i + 1..num_coords {
            let (x2, y2) = coords[j];
            let max_x = x1.max(x2);
            let min_x = x1.min(x2);
            let max_y = y1.max(y2);
            let min_y = y1.min(y2);
            let width = max_x - min_x + 1;
            let height = max_y - min_y + 1;
            let area = width * height;
            if area > largest {
                largest = area;
            }
            if area > contained {
                let mut excluded = false;
                for k in 0..num_coords {
                    let start = coords[k];
                    let end = coords[(k + 1) % num_coords];
                    let diff = (end.0 as i32 - start.0 as i32, end.1 as i32 - start.1 as i32);
                    let excludes = match diff {
                        (0, y_diff) => if y_diff < 0 { // left edge
                            min_x < start.0 && start.0 <= max_x && end.1 < max_y && start.1 > min_y
                        } else { // right edge
                            min_x <= start.0 && start.0 < max_x && start.1 < max_y && end.1 > min_y
                        },
                        (x_diff, 0) => if x_diff < 0 { // bottom edge
                            min_y <= start.1 && start.1 < max_y && end.0 < max_x && start.0 > min_x
                        } else { // top edge
                            min_y < start.1 && start.1 <= max_y && start.0 < max_x && end.0 > min_x
                        },
                        _ => unreachable!()
                    };
                    if excludes {
                        excluded = true;
                        break;
                    }
                }
                if !excluded {
                    contained = area;
                }
            }
        }
    }
    println!("{}", largest);
    println!("{}", contained);
}
