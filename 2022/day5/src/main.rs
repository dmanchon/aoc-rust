const N: usize = 9;
struct Move(usize, usize, usize);

fn parse() -> (Vec<Vec<char>>, Vec<Move>) {
    let input: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();
    let state: Vec<_> = input[0].lines().collect();
    let tallest = state.len() - 1;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; N];

    for stack_no in 0..N {
        let mut tmp = vec![];
        for line_no in 0..tallest {
            let index = stack_no*4 + 1;
            let c = state[line_no].as_bytes()[index];
            if c != b' ' {
                tmp.push(c as char);
            }
        }
        tmp.reverse();
        stacks[stack_no] = tmp;
    }
    for stack_no in 0..N {
        let mut tmp = vec![];
        for line_no in 0..tallest {
            let index = stack_no*4 + 1;
            let c = state[line_no].as_bytes()[index];
            if c != b' ' {
                tmp.push(c as char);
            }
        }
        tmp.reverse();
        stacks[stack_no] = tmp;
    }

    let moves: Vec<Move> = input[1].lines()
        .map(|line| line.split(r" ").collect::<Vec<&str>>())
        .map(|m| Move(m[1].parse::<usize>().unwrap(),
                      m[3].parse::<usize>().unwrap(),
                      m[5].parse::<usize>().unwrap()))
        .collect();

    (stacks, moves)
}

fn part1() {
    let (mut stacks, moves) = parse();
    for Move(n, from, to) in moves {
        for _ in 0..n {
            if let Some(e) = stacks[from-1].pop() {
                stacks[to-1].push(e);                
            } 
        }
    }

    let solution: String = stacks.iter().map(|v| v.last().unwrap()).collect();
    println!("part1: {:#?}", solution);
}

fn part2() {
    let (mut stacks, moves) = parse();
    for Move(n, from, to) in moves {
        let len = stacks[from-1].len();
        for e in stacks[from-1].drain(len-n..).collect::<Vec<_>>() {
            stacks[to-1].push(e);            
        }
    }

    let solution: String = stacks.iter().map(|v| v.last().unwrap()).collect();
    println!("part2: {:#?}", solution);
}

fn main() {
    part1();
    part2();
}
