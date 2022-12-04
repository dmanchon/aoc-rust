use nom::{
    bytes::complete::tag,
    IResult,
};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::character::complete::u32;

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

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, (n1,n2)) = separated_pair(u32, tag("-"), u32)(input)?;
    Ok((input, Range(n1,n2)))
}

fn parse(input: &str) -> IResult<&str, (Range, Range)> {
    let (input, r) = separated_pair(parse_range, tag(","), parse_range)(input)?;
    Ok((input, r))
}

fn part1(ranges: &Vec<(Range, Range)>) {
    let mut count = 0;
    for (x,y) in ranges {
        if x.fully_contains(&y) || y.fully_contains(&x) {
            count += 1;
        }
    }
    println!("part1: {:#?}", count);
}

fn part2(ranges: &Vec<(Range, Range)>) {
    let mut count = 0;
    for (x,y) in ranges {
        if x.overlap(&y) {
            count += 1;
        }
    }
    println!("part2: {:#?}", count);
}

fn main() {
    let (_, ranges) = separated_list1(tag("\n"), parse)(include_str!("../input.txt")).expect("parsing input");
    part1(&ranges);
    part2(&ranges);
}
