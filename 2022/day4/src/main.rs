use nom::{
    multi::separated_list1,
    bytes::complete::tag,
    IResult,
    sequence::separated_pair,
    character::complete::u32,
};

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

fn main() {
    let (_, ranges) = separated_list1(tag("\n"), parse)(include_str!("../input.txt")).expect("parsing input");

    let mut part1 = 0;
    let mut part2 = 0;
    
    for (x,y) in ranges {
        if x.overlap(&y) {
            part2 += 1;
        }
        if x.fully_contains(&y) || y.fully_contains(&x) {
            part1 += 1;
        }
    }

    println!("part1: {:#?}", part1);
    println!("part2: {:#?}", part2);
}
