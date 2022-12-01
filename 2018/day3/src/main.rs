use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

use nom::{
    bytes::complete::tag,
    character::{
        complete::digit1,
        complete::{alphanumeric1, char},
    },
    combinator::map,
    combinator::map_res,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Claim<'a> {
    id: &'a str,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input)
}

impl<'a> Claim<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            tuple((
                tag("#"),
                alphanumeric1,
                tag(" @ "),
                separated_pair(parse_number, char(','), parse_number),
                tag(": "),
                separated_pair(parse_number, char('x'), parse_number),
            )),
            |(_, id, _, (x, y), _, (width, height))| Self {
                id,
                x,
                y,
                width,
                height,
            },
        )(input)
    }

    fn points(&self) -> Vec<(u32, u32)> {
        let mut res = vec![];
        for i in 0..self.width {
            for j in 0..self.height {
                res.push((self.x + i, self.y + j))
            }
        }
        res
    }
}

fn part1() {
    let mut points = HashMap::<(u32, u32), u32>::new();
    let mut claims = vec![];
    if let Ok(input) = fs::read_to_string("input.txt") {
        for line in input.lines() {
            claims.push(Claim::parse(line).expect("Fail to parse").1);
        }
        for claim in &claims {
            for p in claim.points() {
                if let Some(c) = points.get_mut(&p) {
                    *c += 1;
                } else {
                    points.insert(p, 1);
                }
            }
        }

        let result = points.values().filter(|&&x| x > 1).count();
        println!("Part1: {:#?}", result);
    }
}

fn part2() {
    let mut points = HashMap::<(u32, u32), Vec<&str>>::new();
    let mut claims = vec![];
    if let Ok(input) = fs::read_to_string("input.txt") {
        for line in input.lines() {
            claims.push(Claim::parse(line).expect("Fail to parse").1);
        }
        for claim in &claims {
            for p in claim.points() {
                if let Some(v) = points.get_mut(&p) {
                    v.push(claim.id);
                } else {
                    points.insert(p, vec![claim.id]);
                }
            }
        }

        let mut overlaps: HashSet<&str> = HashSet::new();
        for p in points.values().filter(|x| x.len() == 1).flatten() {
            overlaps.insert(p);
        }
        for p in points.values().filter(|x| x.len() > 1).flatten() {
            overlaps.remove(p);
        }

        println!("Part2: {:#?}", overlaps.into_iter().take(1).next().unwrap());
    }
}

fn main() {
    part1();
    part2();
}
