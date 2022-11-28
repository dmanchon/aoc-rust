use std::fs;


#[derive(Debug)]
struct Id<'a> {
    counts: [u8; 26],
    repr: &'a str,
}


impl<'a> Id<'a> {
    fn new(s: &'a str) -> Id {
        let mut res = [0;26];
        for c in s.bytes() {
            let idx = c - 0x61;
            res[idx as usize] += 1;
        }
        Self{counts: res, repr: s}
    }

    fn has_pairs(&self) -> bool {
        self.counts.iter().filter(|&&x| x==2).count() > 0
    }
    
    fn has_triplets(&self) -> bool {
        self.counts.iter().filter(|&&x| x==3).count() > 0
    }

    fn diff(&self, other: &Self) -> usize {
        let mut dist = 0;
        for i in 0..self.repr.len() {
            if self.repr.as_bytes()[i] != other.repr.as_bytes()[i] {
                dist += 1;
            }
        }
        dist
    }
}

fn part1() {
    let mut twos = 0;
    let mut threes = 0;
    
    let input = fs::read_to_string("input.txt").expect("reading the input");
    for line in input.lines() {
        let id = Id::new(line);
        if id.has_pairs() {
            twos += 1;
        }
        if id.has_triplets() {
            threes += 1;
        }
    }

    println!("Part1: {:#?}", twos*threes);
}

fn part2() {
    let mut ids = vec![];
    let input = fs::read_to_string("input.txt").expect("reading the input");
    for line in input.lines() {
        let id = Id::new(line);
        ids.push(id);
    }

    'outer: for id1 in &ids {
        for id2 in &ids {
            if id1.diff(id2) == 1 {
                println!("Part2: {:#?} vs {:#?}", id1.repr, id2.repr);
                break 'outer;
            } 
        }
    }
}


fn main() {
    part1();
    part2();
}
