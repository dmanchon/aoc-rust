type OpFn = fn(u128) -> u128;

struct Monkey {
    items: Vec<u128>,
    operation: OpFn,
    test: u128,
    destinations: (usize, usize),
    moves: u128,
}

fn main() {
    println!("part1: {:#?}", solve(20, 3));
    println!("part2: {:#?}", solve(10_000, 1));
}

fn solve(n: usize, div: u128) -> u128 {
    let mut monkeys = [
        Monkey{
            items: vec![
                92,
                73,
                86,
                83,
                65,
                51,
                55,
                93,
            ],
            operation: |x| x*5,
            test: 11,
            destinations: (3, 4),
            moves: 0,
        },
        Monkey{
            items: vec![
                99,
                67,
                62,
                61,
                59,
                98,
            ],
            operation: |x| x*x,
            test: 2,
            destinations: (6, 7),
            moves: 0,
        },
        Monkey{
            items: vec![
                81,
                89,
                56,
                61,
                99,
            ],
            operation: |x| x*7,
            test: 5,
            destinations: (1, 5),
            moves: 0,
        },
        Monkey{
            items: vec![
                97,
                74,
                68,
            ],
            operation: |x| x+1,
            test: 17,
            destinations: (2, 5),
            moves: 0,
        },
        Monkey{
            items: vec![
                78,
                73,
            ],
            operation: |x| x+3,
            test: 19,
            destinations: (2, 3),
            moves: 0,
        },
        Monkey{
            items: vec![
                50,
            ],
            operation: |x| x+5,
            test: 7,
            destinations: (1, 6),
            moves: 0,
        },
        Monkey{
            items: vec![
                95,
                88,
                53,
                75,
            ],
            operation: |x| x+8,
            test: 3,
            destinations: (0, 7),
            moves: 0,
        },
        Monkey{
            items: vec![
                50,
                77,
                98,
                85,
                94,
                56,
                89,
            ],
            operation: |x| x+2,
            test: 13,
            destinations: (4, 0),
            moves: 0,
        },

    ];
    let lcm: u128 = monkeys.iter().map(|m|m.test).product();

    for _ in 0..n {
        for i in 0..monkeys.len() {
            // items with fn applied
            let items: Vec<u128> = monkeys[i].items
                .iter()
                .map(|&x| {
                    if div > 1 {
                        (monkeys[i].operation)(x) / div
                    } else {    
                        (monkeys[i].operation)(x) % lcm
                    }
                }).collect();
            
            // how many has handled
            monkeys[i].moves += items.len() as u128;

            // pass it
            for item in items {
                if item%monkeys[i].test == 0 {
                    monkeys[monkeys[i].destinations.0].items.push(item);
                } else {
                    monkeys[monkeys[i].destinations.1].items.push(item);
                }
            }
            monkeys[i].items.clear();
        }
    }

    let mut moves: Vec<_> = monkeys.iter().map(|m| m.moves).collect();
    moves.sort();
    moves.reverse();
    moves[0] * moves[1]
}