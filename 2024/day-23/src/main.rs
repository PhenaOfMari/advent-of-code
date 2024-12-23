use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input_file.lines() {
        let mut split = line.split("-");
        let a = split.next().unwrap();
        let b = split.next().unwrap();
        let a_map = connections.get_mut(a);
        if a_map.is_some() {
            a_map.unwrap().push(b);
        } else {
            connections.insert(a, vec![b]);
        }
        let b_map = connections.get_mut(b);
        if b_map.is_some() {
            b_map.unwrap().push(a);
        } else {
            connections.insert(b, vec![a]);
        }
    }
    let complete_clusters = get_complete_clusters(&connections);

    let mut clusters3 = HashSet::new();
    for cluster in complete_clusters.iter() {
        let len = cluster.len();
        for i in 0..len - 2 {
            for j in i + 1..len - 1 {
                for k in j + 1..len {
                    if cluster[i].starts_with('t') || cluster[j].starts_with('t') || cluster[k].starts_with('t') {
                        clusters3.insert(vec![cluster[i], cluster[j], cluster[k]].join(","));
                    }
                }
            }
        }
    }
    println!("{}", clusters3.len());

    let mut longest = Vec::new();
    for cluster in complete_clusters.iter() {
        if cluster.len() > longest.len() {
            longest = cluster.clone();
        }
    }
    println!("{}", longest.join(","));
}

fn get_complete_clusters<'a>(connections: &HashMap<&'a str, Vec<&'a str>>) -> HashSet<Vec<&'a str>> {
    let mut clusters = HashSet::new();
    for (&a, a_adj) in connections.iter() {
        let mut cluster = vec![a];
        let mut queue = a_adj.clone();
        while !queue.is_empty() {
            let b = queue.pop().unwrap();
            let b_adj = connections.get(b).unwrap();
            if cluster.len() == 1 && !b_adj.iter().any(|c| queue.contains(c)) {
                continue;
            }
            cluster.push(b);
            queue = queue.iter().filter(|c| b_adj.contains(c)).cloned().collect();
        }
        if cluster.len() > 2 {
            cluster.sort();
            clusters.insert(cluster);
        }
    }
    clusters
}
