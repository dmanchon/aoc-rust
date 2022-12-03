use std::collections::HashSet;

fn calc_score(item: char) -> u32 {
    if item.is_lowercase() {
        item as u32 - '`' as u32
    } else {
        item as u32 - '@' as u32 + 26
    }
}

fn part1() {
    let mut solution: u32 = 0;
    for line in include_str!("../input.txt").lines() {
        let (l, r) = line.split_at(line.len() / 2);
        let a: HashSet<char> = HashSet::from_iter(l.chars());
        let b: HashSet<char> = HashSet::from_iter(r.chars());

        if let Some(&c) = a.intersection(&b).next() {
            solution += calc_score(c);
        };
    }
    println!("part1: {:#?}", solution);
}

fn part2() {
    let mut solution: u32 = 0;
    let mut lines = vec![];
    for line in include_str!("../input.txt").lines() {
        lines.push(line);
    }

    for chunk in lines.chunks(3) {
        let mut first: HashSet<char> = HashSet::from_iter(chunk[0].chars());
        let second: HashSet<char> = HashSet::from_iter(chunk[1].chars());
        let third: HashSet<char> = HashSet::from_iter(chunk[2].chars());

        first.retain(|element| {
            for s in [&second, &third] {
                if !s.contains(element) {
                    return false;
                }
            }
            true
        });

        if let Some(&c) = first.iter().next() {
            solution += calc_score(c);
        };
    }
    println!("part2: {:#?}", solution);
}

fn part2_idiomatic() {
    // sort of idiomatic, still trying to learn Rust
    // is this really more idiomatic? or readable?
    let solution: u32 = include_str!("../input.txt")
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .map(|c| c.chars())
                .map(|x| HashSet::<char>::from_iter(x))
                .reduce(|mut a, b| {
                    a.retain(|x| b.contains(x));
                    a
                })
                .map_or(0, |x| calc_score(*x.iter().next().unwrap()))
        })
        .sum();

    println!("part2: {:#?}", solution);
}

fn main() {
    part1();
    part2();
    part2_idiomatic()
}
