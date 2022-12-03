use std::collections::HashSet;

fn calc_score(item: u8) -> u16 {
    if item.is_ascii_lowercase() {
        item as u16 - '`' as u16
    } else {
        item as u16 - '@' as u16 + 26
    }
}

fn get_common(first: &[u8], second: &[u8], third: &[u8]) -> u8 {
    for ch in first {
        if second.contains(ch) && third.contains(ch) {
            return *ch;
        }
    };
    panic!()
}

fn part2() {
    let mut solution: u16 = 0;
    let mut lines = vec![];
    for line in include_str!("../input.txt").lines() {
        lines.push(line);
    }
    
    for chunk in lines.chunks(3) {
        let first = chunk[0].as_bytes();
        let second = chunk[1].as_bytes();
        let third = chunk[2].as_bytes();

        solution += calc_score(get_common(first, second, third));
       
    }
    println!("part2: {:#?}", solution);
}

fn part1() {
    let mut solution: u16 = 0;
    for line in include_str!("../input.txt").lines() {
        let (l, r) = line.split_at(line.len() / 2);
        let a: HashSet<char> = HashSet::from_iter(l.chars());
        let b: HashSet<char> = HashSet::from_iter(r.chars());

        if let Some(&c) = a.intersection(&b).next() {
            solution += calc_score(c as u8);
        };
    }
    println!("part1: {:#?}", solution);
}

fn main() {
    part1();
    part2();
}
