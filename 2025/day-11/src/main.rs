use std::collections::HashMap;
use std::fs;

type Data = (usize, usize, usize, usize);

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut connections = HashMap::new();
    for line in input_file.lines() {
        let mut tokens = line.split_whitespace();
        let input = &tokens.next().unwrap()[..3];
        let mut output = Vec::new();
        while let Some(token) = tokens.next() {
            output.push(token);
        }
        connections.insert(input, output);
    }

    let mut memo = HashMap::new();
    let paths1 = find_paths(&connections, "you", &mut memo);
    println!("{}", paths1.0);
    let paths2 = find_paths(&connections, "svr", &mut memo);
    println!("{:?}", paths2.1);
}

fn find_paths<'a>(connections: &HashMap<&str, Vec<&'a str>>, input: &'a str, memo: &mut HashMap<&'a str, Data>) -> Data {
    if input == "out" {
        (1, 0, 0, 0)
    } else if let Some(result) = memo.get(input) {
        *result
    } else {
        let mut result = (0, 0, 0, 0);
        for &output in connections.get(&input).unwrap().iter() {
            let data = find_paths(connections, output, memo);
            result.0 += data.0;
            result.1 += data.1;
            result.2 += data.2;
            result.3 += data.3;
        }
        if input == "dac" {
            result.1 += result.3;
            result.2 = result.0;
        } else if input == "fft" {
            result.1 += result.2;
            result.3 = result.0;
        }
        memo.insert(input, result);
        result
    }
}
