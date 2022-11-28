use std::{fs, str::FromStr, iter::repeat, collections::BTreeSet};


fn part1() {
    let mut freq = 0;
    if let Ok(input) = fs::read_to_string("input.txt") {
        for line in input.lines() {
            freq += i32::from_str(line).expect("Error parsing");
        }
        println!("Part1: {}", freq);
    }
}

fn part2() {
    let mut changes = vec![];
    let mut freqs = BTreeSet::new();
    let mut freq = 0;
    if let Ok(input) = fs::read_to_string("input.txt") {
        for line in input.lines() {
            changes.push(i32::from_str(line).expect("Error parsing"));
        }
        for f in repeat(&changes).flat_map(|it| it.iter()) {
            freq += f;
            if freqs.contains(&freq) {
                println!("Part2: {}",freq);
                break;
            } else {
                freqs.insert(freq);
            }
        }
    }
}

fn main() {
    part1();
    part2();
}
