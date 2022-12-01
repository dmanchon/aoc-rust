use std::str::FromStr;

fn main() {
    let mut elves = vec![];
    let mut elf = 0;
    let input = std::fs::read_to_string("input.txt").expect("reading input file");

    for line in input.lines() {
        if line == "" {
            elves.push(elf);
            elf = 0;
        } else {
            elf += u32::from_str(line).expect("string -> u32");
        }
    }
    elves.sort();
    elves.reverse();

    println!("part1: {:#?}", elves[0]);
    println!("part2: {:#?}", elves[0..3].to_vec().iter().sum::<u32>());
}
