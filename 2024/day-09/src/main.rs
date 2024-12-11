use std::fs;

fn main() {
    let input_file = fs::read_to_string("input").expect("Should be able to read file");
    let compressed = input_file.chars()
        .filter(|&c| c != '\n')
        .map(|c| c.to_digit(10).unwrap() as u16)
        .collect::<Vec<u16>>();
    let mut file_index = 0;
    let decompressed = compressed.iter().enumerate()
        .flat_map(|(i, &n)| {
            let indicator = if i % 2 == 1 {
                u16::MAX
            } else {
                let index = file_index;
                file_index += 1;
                index
            };
            let mut group = Vec::new();
            for _ in 0..n {
                group.push(indicator);
            }
            group
        })
        .collect::<Vec<u16>>();

    let mut compacted = decompressed.clone();
    let mut i = 0;
    let mut j = compacted.len() - 1;
    while i < j {
        if compacted[i] == u16::MAX {
            while compacted[j] == u16::MAX && j > i {
                j -= 1;
            }
            compacted.swap(i, j);
        }
        i += 1;
    }
    let compact_checksum = checksum(&compacted);
    println!("{}", compact_checksum);

    let mut defragmented = decompressed.clone();
    let mut file_index = Vec::new();
    let mut space_index = Vec::new();
    let mut k = 0;
    while k < decompressed.len() {
        let mut size = 1;
        while k + size < defragmented.len() && defragmented[k + size] == defragmented[k] {
            size += 1;
        }
        if defragmented[k] == u16::MAX {
            space_index.push((k, size));
        } else {
            file_index.push((k, size));
        }
        k += size
    }
    for file in file_index.iter().rev() {
        for (l, space) in space_index.clone().iter().enumerate() {
            if space.0 > file.0 {
                break;
            }
            if space.1 >= file.1 {
                let new_size = space.1 - file.1;
                space_index.remove(l);
                if new_size > 0 {
                    space_index.insert(l, (space.0 + file.1, new_size));
                }
                for m in 0..file.1 {
                    defragmented.swap(space.0 + m, file.0 + m);
                }
                break;
            }
        }
    }
    let defragmented_checksum = checksum(&defragmented);
    println!("{}", defragmented_checksum);
}

fn checksum(input: &Vec<u16>) -> u64 {
    input.iter().enumerate().fold(0, |sum, (i, &n)| {
        if n == u16::MAX {
            sum
        } else {
            sum + i as u64 * n as u64
        }
    })
}
