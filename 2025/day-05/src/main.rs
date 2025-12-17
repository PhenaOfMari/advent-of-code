use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();
    let mut found_break = false;
    for line in input_file.lines() {
        if line.is_empty() {
            found_break = true;
        } else if found_break {
            ingredients.push(line.parse::<usize>().unwrap());
        } else {
            let mut tokens = line.split("-");
            let start = tokens.next().unwrap().parse::<usize>().unwrap();
            let end = tokens.next().unwrap().parse::<usize>().unwrap();
            ranges.push((start, end));
        }
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut combined = vec![ranges[0]];
    for (start, end) in ranges {
        let i = combined.len() - 1;
        if start <= combined[i].1 {
            combined[i].1 = end.max(combined[i].1);
        } else {
            combined.push((start, end));
        }
    }

    let fresh = ingredients.iter()
        .filter(|&i| combined.iter().any(|(start, end)| start <= i && i <= end))
        .count();
    println!("{}", fresh);

    let total_fresh = combined.iter().fold(0, |sum, (start, end)| sum + end - start + 1);
    println!("{}", total_fresh);
}
