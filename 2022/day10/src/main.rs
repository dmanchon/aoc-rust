
#[derive(Debug)]
enum Inst {
    Noop,
    Addx(i32),
}

fn parse(input: &str) -> Vec<Inst> {
    let mut result = vec![];
    for line in input.lines() {
        match line {
            "noop" => result.push(Inst::Noop),
            _ => {
                let n: &str = line.split(r" ").skip(1).take(1).next().unwrap();
                result.push(Inst::Addx(n.parse().unwrap()));
            }
        }
    }
    result
}

fn main() {
    let mut states: Vec<i32> = vec![1];
    for inst in parse(include_str!("../input.txt")) {
        match inst {
            Inst::Noop => states.push(0),
            Inst::Addx(n) => states.extend([0, n]),
        }
    }

    let part1: i32 = states.iter()
        .scan(0, |acc, &x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .enumerate()
        .filter(|&(i, _)| [19, 59, 99, 139, 179, 219].contains(&i))
        .map(|(i, x)| (i+1) as i32 * x).sum();
    println!("part1: {}", part1);

    let part2: Vec<u8> = states.iter()
        .scan(0, |acc, &x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .enumerate()
        .map(|(i, x)| {
            match i32::abs(x%40 - (i as i32)%40) {
                v if v < 2 => b'#',
                _ => b'.'
            }
        }).collect();

    println!("part2:");
    for i in 0..part2.len()-1 {
        if i%40==0 {
            print!("\n");
        }
        print!("{}", part2[i] as char);
    }
}
