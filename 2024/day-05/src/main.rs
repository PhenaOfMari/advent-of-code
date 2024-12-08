use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let rules = input_file.lines()
        .filter(|line| line.contains('|'))
        .map(|line| {
            let items = line.split('|').map(|item| item.parse::<u8>().unwrap()).collect::<Vec<u8>>();
            (items[0], items[1])
        })
        .collect::<Vec<(u8, u8)>>();
    let updates = input_file.lines()
        .filter(|line| line.contains(','))
        .map(|line| line.split(',').map(|item| item.parse::<u8>().unwrap()).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let num_correct = updates.iter().fold(0, |sum, update| {
        if rules.iter().any(|rule| check_rule_failure(&rule, update)) {
            sum
        } else {
            sum + update[update.len() / 2] as u16
        }
    });
    println!("{}", num_correct);

    let sum_incorrect = updates.iter().fold(0, |sum, update| {
        let filtered_rules = rules.iter()
            .filter(|rule| update.contains(&rule.0) && update.contains(&rule.1))
            .collect::<Vec<&(u8, u8)>>();

        let mut clean = true;
        let mut corrected_update = update.clone();
        let mut maybe_rule = filtered_rules.iter().find(|rule| check_rule_failure(rule, &corrected_update));
        while maybe_rule.is_some() {
            clean = false;
            let rule = maybe_rule.unwrap();
            let a = corrected_update.iter().position(|&item| item == rule.0).unwrap();
            let b = corrected_update.iter().position(|&item| item == rule.1).unwrap();
            let temp = corrected_update[a];
            corrected_update[a] = corrected_update[b];
            corrected_update[b] = temp;
            maybe_rule = filtered_rules.iter().find(|rule| check_rule_failure(rule, &corrected_update));
        }
        if clean {
            sum
        } else {
            sum + corrected_update[corrected_update.len() / 2] as u16
        }
    });
    println!("{}", sum_incorrect);
}

fn check_rule_failure(rule: &&(u8, u8), update: &Vec<u8>) -> bool {
    update.iter().position(|&item| item == rule.0)
        .is_some_and(|a| update.iter().position(|&item| item == rule.1)
            .is_some_and(|b| b < a))
}