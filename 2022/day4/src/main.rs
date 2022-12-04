use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    combinator::map_res,
    sequence::tuple,
    IResult,
};
use std::str::FromStr;

#[derive(Debug)]
struct Range(u32,u32);

impl Range {
    fn fully_contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
    
    fn overlap(&self, other: &Self) -> bool {
        self.0 <= other.1 && self.1 >= other.0
    }
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input)
}

fn parse(input: &str) -> IResult<&str, (Range, Range)> {
    map(
        tuple(
            (parse_number, tag("-"), parse_number, tag(","), parse_number, tag("-"), parse_number)
        ),
        |(x1, _, x2, _, y1, _, y2)| (Range(x1,x2), Range(y1,y2))
    )(input)
}

fn part1() {
    let mut count = 0;
    for line in include_str!("../input.txt").lines() {
        let (x,y) = parse(line).expect("problem parsing line").1;
        if x.fully_contains(&y) || y.fully_contains(&x) {
            count += 1;
        }
    }
    println!("part1: {:#?}", count);
}

fn part2() {
    let mut count = 0;
    for line in include_str!("../input.txt").lines() {
        let (x,y) = parse(line).expect("").1;
        if x.overlap(&y) {
            count += 1;
        }
    }
    println!("part2: {:#?}", count);
}

fn main() {
    part1();
    part2();
}
