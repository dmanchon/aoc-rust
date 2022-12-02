use std::ops::AddAssign;

struct Tuple(u32, u32);
impl AddAssign for Tuple {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}
fn main() {
    let input = std::fs::read_to_string("input.txt").expect("reading input file");
    let mut total = Tuple(0, 0);
    for line in input.lines() {
        total += match line {
            "A X" => Tuple(0 + 3, 3 + 1),
            "A Y" => Tuple(3 + 1, 6 + 2),
            "A Z" => Tuple(6 + 2, 0 + 3),
            "B X" => Tuple(0 + 1, 0 + 1),
            "B Y" => Tuple(3 + 2, 3 + 2),
            "B Z" => Tuple(6 + 3, 6 + 3),
            "C X" => Tuple(0 + 2, 6 + 1),
            "C Y" => Tuple(3 + 3, 0 + 2),
            "C Z" => Tuple(6 + 1, 3 + 3),
            _ => panic!("not valid input"),
        };
    }
    println!("part1: {:#?}", total.1);
    println!("part2: {:#?}", total.0);
}
