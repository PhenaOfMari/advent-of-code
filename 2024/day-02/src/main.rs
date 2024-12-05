use std::cmp::Ordering;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let reports = input_file.lines()
        .map(|line| line.split_whitespace().map(|token| token.parse().unwrap()).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let safe_reports = reports.iter().filter(check_safety).count();
    println!("{}", safe_reports);

    let dampened_reports = reports.iter()
        .filter(|report| {
            check_safety(report) || report.iter().enumerate().any(|(index, _)| {
                let mut dampened = (*report).clone();
                dampened.remove(index);
                check_safety(&&dampened)
            })
        })
        .count();
    println!("{}", dampened_reports);
}

fn check_safety(report: &&Vec<u8>) -> bool {
    match report[0].cmp(&report[1]) {
        Ordering::Equal => false,
        Ordering::Greater => {
            for i in 1..report.len() {
                let safe = report[i - 1] > report[i] && report[i - 1] - report[i] < 4;
                if !safe {
                    return false
                }
            }
            true
        },
        Ordering::Less => {
            for i in 1..report.len() {
                let safe = report[i - 1] < report[i] && report[i] - report[i - 1] < 4;
                if !safe {
                    return false
                }
            }
            true
        }
    }
}