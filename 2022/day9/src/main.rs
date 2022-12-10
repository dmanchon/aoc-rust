use std::collections::HashSet;
use num_complex::Complex;

#[derive(Clone, Copy, Debug)]
enum Move {
    R, L, U, D,
}

fn parse(input: &str) -> Vec<Move> {
    input.lines().map(|line| {
        let n: usize = line[2..].parse().unwrap();
        match &line[0..2] {
            "R " => [Move::R].repeat(n),
            "L " => [Move::L].repeat(n),
            "U " => [Move::U].repeat(n),
            "D " => [Move::D].repeat(n),
            _ => panic!("error parsing input"),
        }
    }).flatten().collect()   
}

fn move_head(cmd: Move, head: Complex<i32>) -> Complex<i32> {
    match cmd {
        Move::D => head + Complex::<i32>::new(0, -1),
        Move::U => head + Complex::<i32>::new(0, 1),
        Move::L => head + Complex::<i32>::new(-1, 0),
        Move::R => head + Complex::<i32>::new(1, 0),
    }
}

fn move_tail(head: Complex<i32>, tail: Complex<i32>) -> Complex<i32> {
    let diff = head - tail;
    if diff.norm_sqr() > 2 {
        tail + Complex::<i32>::new(diff.re.signum(), diff.im.signum())
    } else {
        tail
    }
}

fn main() {
    let mut rope = vec![Complex::<i32>::new(0, 0); 10];
    let mut part1 = HashSet::new();
    let mut part2 = HashSet::new();
    
    for cmd in parse(include_str!("../input.txt")) {
        rope[0] = move_head(cmd, rope[0]);
        for i in 1..rope.len() {
           rope[i] = move_tail(rope[i-1], rope[i]);            
        }

        part1.insert(rope[1].clone());
        if let Some(last) = rope.last().cloned() {
            part2.insert(last);
        }        
    }
    println!("part1: {}", part1.len());        
    println!("part2: {}", part2.len());        
}
