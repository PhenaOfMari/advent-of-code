use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut ranges = Vec::new();
    for line in input_file.lines() {
        for chunk in line.split(',') {
            let mut tokens = chunk.split('-');
            let start = tokens.next().unwrap().parse::<usize>().unwrap();
            let end = tokens.next().unwrap().parse::<usize>().unwrap();
            ranges.push((start, end));
        }
    }

    let mut sum1 = 0;
    let mut sum2 = 0;
    for (start, end) in ranges {
        'outer: for i in start..=end {
            let digits = i.ilog10() + 1;
            let half = digits >> 1;
            let mut d = half;
            while d > 0 {
                if digits % d == 0 {
                    let divisor = 10usize.pow(d);
                    let rem = i % divisor;
                    let mut checks = Vec::new();
                    let mut j = i / divisor;
                    while j > 0 {
                        checks.push(j % divisor);
                        j /= divisor;
                    }
                    if checks.iter().all(|&n| n == rem) {
                        if digits & 1 == 0 && d == half {
                            sum1 += i;
                        }
                        sum2 += i;
                        continue 'outer;
                    }
                }
                d -= 1;
            }
        }
    }
    println!("{}", sum1);
    println!("{}", sum2);
}
