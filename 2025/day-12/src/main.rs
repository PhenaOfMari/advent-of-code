use std::fs;

struct Region {
    width: usize,
    height: usize,
    req: [usize; 6]
}

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut shapes = Vec::new();
    let mut regions = Vec::new();
    let mut empty_found = 0;
    let mut row_num = 0;
    for line in input_file.lines() {
        if line.is_empty() {
            empty_found += 1;
        } else if empty_found >= 6 {
            let mut tokens = line.split_whitespace();
            let size = tokens.next().unwrap();
            let mut size_tokens = size[0..size.len() - 1].split("x");
            let width = size_tokens.next().unwrap().parse().unwrap();
            let height = size_tokens.next().unwrap().parse().unwrap();
            let mut req = [0; 6];
            for i in 0..6 {
                req[i] = tokens.next().unwrap().parse().unwrap();
            }
            regions.push(Region { width, height, req });
        } else {
            if line.ends_with(":") {
                shapes.push([[' '; 3]; 3]);
                row_num = 0;
            } else {
                for i in 0..3 {
                    shapes.last_mut().unwrap()[row_num][i] = line.chars().nth(i).unwrap();
                }
                row_num += 1;
            }
        }
    }

    let mut fitting = 0;
    for region in regions.iter() {
        let mut grid = vec![vec![' '; region.width]; region.height];
        let mut remaining = region.req.clone();
        'outer: for i in 0..region.height - 2 {
            'next: for j in 0..region.width - 2 {
                if remaining.iter().all(|&x| x == 0) {
                    break 'outer;
                }
                for (shape_id, mut shape) in shapes.iter().cloned().enumerate() {
                    if remaining[shape_id] == 0 {
                        continue;
                    }
                    let mut rotates = 0;
                    let mut flips = 0;
                    while flips < 2 {
                        let mut fits = true;
                        'check: for k in 0..3 {
                            for l in 0..3 {
                                if shape[k][l] == '#' && grid[i+k][j+l] == '#' {
                                    fits = false;
                                    break 'check;
                                }
                            }
                        }
                        if fits {
                            for k in 0..3 {
                                for l in 0..3 {
                                    if shape[k][l] == '#' {
                                        grid[i+k][j+l] = '#';
                                    }
                                }
                            }
                            remaining[shape_id] -= 1;
                            continue 'next;
                        }
                        rotate(&mut shape);
                        rotates += 1;
                        if rotates > 3 {
                            rotates = 0;
                            flip(&mut shape);
                            flips += 1;
                        }
                    }
                }
            }
        }
        if remaining.iter().all(|&x| x == 0) {
            fitting += 1;
        }
    }
    println!("{}", fitting);
}

fn rotate(shape: &mut [[char; 3]; 3]) {
    let clone = shape.clone();
    for i in 0..3 {
        for j in 0..3 {
            shape[i][j] = clone[2-j][i]
        }
    }
}

fn flip(shape: &mut [[char; 3]; 3]) {
    for i in 0..3 {
        shape[i].reverse();
    }
}
