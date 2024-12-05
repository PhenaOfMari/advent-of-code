use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let lines = input_file.lines();
    let line_count = lines.clone().count();
    let mut list1 = Vec::with_capacity(line_count);
    let mut list2 = Vec::with_capacity(line_count);
    for line in lines {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        list1.push(tokens[0].parse::<u32>().unwrap());
        list2.push(tokens[1].parse::<u32>().unwrap());
    };
    list1.sort();
    list2.sort();
    let distance = list1.iter().zip(list2.iter()).fold(0, |sum, item| {
        sum + if item.0 < item.1 {
            item.1 - item.0
        } else {
            item.0 - item.1
        }
    });
    println!("{}", distance);
    let similarity = list1.iter().fold(0, |sum, item1| {
        sum + item1 * list2.iter().filter(|item2| item1 == *item2).count() as u32
    });
    println!("{}", similarity);
}
