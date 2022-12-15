use std::cmp::Ordering;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    *,
};


#[derive(Debug, Eq)]
enum Packet {
    Array(Vec<Packet>),
    Number(u32),
}


impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Array(a1), Packet::Array(a2)) => a1 == a2,
            (Packet::Array(a1), Packet::Number(n2)) => a1 == &vec![Packet::Number(*n2)],
            (Packet::Number(n1), Packet::Array(a2)) => &vec![Packet::Number(*n1)] == a2,
            (Packet::Number(n1), Packet::Number(n2)) => n1 == n2,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Array(a1), Packet::Array(a2)) => a1.cmp(a2),
            (Packet::Array(a1), Packet::Number(n2)) => a1.cmp(&vec![Packet::Number(*n2)]),
            (Packet::Number(n1), Packet::Array(a2)) => vec![Packet::Number(*n1)].cmp(a2),
            (Packet::Number(n1), Packet::Number(n2)) => n1.cmp(n2),            
        }
    }
}

fn parse(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(
            tag("["),
            separated_list0(tag(","), parse),
            tag("]"),
        ).map(|vec| Packet::Array(vec)),
        nom::character::complete::u32.map(|num| Packet::Number(num)),
    ))(input)
}


fn main() {
    let pairs = separated_list1(tag("\n\n"), separated_pair(parse, newline, parse))(include_str!("../input.txt")).unwrap().1;

    let mut sum = 0;
    for (i, (a, b)) in pairs.iter().enumerate() {
        if b >= a {
            sum += i+1;
        }
    }
    println!("part1: {:#?}", sum);

    let p6 = Packet::Array(vec![Packet::Array(vec![Packet::Number(6)])]);
    let p2 = Packet::Array(vec![Packet::Array(vec![Packet::Number(2)])]);
    let mut packets = vec![&p2, &p6];
    
    for (a, b) in pairs.iter() {
        packets.push(a);
        packets.push(b);
    }

    packets.sort();
    let pos6 = packets.iter().position(|&packet| packet == &p6).unwrap();
    let pos2 = packets.iter().position(|&packet| packet == &p2).unwrap();
    let part2 = (pos2+1) * (pos6+1);
    println!("part2: {:#?}", part2);
}
