use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let (mut list1, mut list2): (Vec<u32>, Vec<u32>) = input_file.lines()
        .map(|line| {
            let tokens = line.split_whitespace().map(|token| token.parse().unwrap()).collect::<Vec<u32>>();
            (tokens[0], tokens[1])
        })
        .unzip();
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
