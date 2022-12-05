fn parse() -> (Vec<Vec<char>>, &'static str) {
    let input: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();

    let state: Vec<_> = input[0].lines().collect();
    let tallest = state.len() - 1;
    const N: usize = 9;
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

    (stacks, input[1])
}

fn part1() {
    let (mut stacks, moves) = parse();
    for move_str in moves.lines() {
        let mov: Vec<_> = move_str.split(r" ").collect();
        let (n, from, to) = (mov[1].parse::<usize>().unwrap(), mov[3].parse::<usize>().unwrap(), mov[5].parse::<usize>().unwrap());

        for _ in 0..n {
            if let Some(e) = stacks[from-1].pop() {
                stacks[to-1].push(e);                
            } 
        }
    }

    let mut solution: Vec<&char> = vec![];
    for s in &stacks {
        solution.push(s.last().unwrap());
        
    }
    println!("part1: {:#?}", solution);
}

fn part2() {
    let (mut stacks, moves) = parse();
    for move_str in moves.lines() {
        let mov: Vec<_> = move_str.split(r" ").collect();
        let (n, from, to) = (mov[1].parse::<usize>().unwrap(), mov[3].parse::<usize>().unwrap(), mov[5].parse::<usize>().unwrap());

        let len = stacks[from-1].len();
        for e in stacks[from-1].drain(len-n..).collect::<Vec<_>>() {
            stacks[to-1].push(e);            
        }

    }

    let mut solution: Vec<&char> = vec![];
    for s in &stacks {
        solution.push(s.last().unwrap());
        
    }
    println!("part2: {:#?}", solution);
}

fn main() {
    part1();
    part2();
}
