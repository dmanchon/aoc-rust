use std::collections::{HashSet, VecDeque};

fn solve(n: usize) -> usize {
    let mut header: VecDeque<&u8> = VecDeque::new();
    let mut solution: usize = 0;
    
    for c in include_bytes!("../input.txt") {
        solution += 1;
        header.push_back(c);
        if header.len() == n {
            if header.iter().collect::<HashSet<_>>().len() == n {
                break;
            }
            header.pop_front();
        }
    }
    solution
}

fn main() {
    println!("part1: {:#?}", solve(4));
    println!("part2: {:#?}", solve(14));
}
