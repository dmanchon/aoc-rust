use std::collections::BTreeSet;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::i32 as parse_i32;
use nom::multi::separated_list1;
use nom::character::complete::newline;

fn line(s: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    let (s, _) = tag("Sensor at x=")(s)?;
    let (s, sx) = parse_i32(s)?;
    let (s, _) = tag(", y=")(s)?;
    let (s, sy) = parse_i32(s)?;
    let (s, _) = tag(": closest beacon is at x=")(s)?;
    let (s, bx) = parse_i32(s)?;
    let (s, _) = tag(", y=")(s)?;
    let (s, by) = parse_i32(s)?;

    Ok((s, ((sx, sy),(bx, by))))
}

fn parse(s: &str) -> IResult<&str, Vec<((i32, i32), (i32, i32))>> {
    separated_list1(newline, line)(s)
}

fn part1(target_y: i32) -> BTreeSet<i32> {
    let state = parse(include_str!("../input.txt")).expect("parsing").1;
    
    let mut positions = state.iter().filter_map(|((sx,sy),(bx,by))| {
        let radius = (sx-bx).abs()+(sy-by).abs();
        let delta_y = (target_y - sy).abs();

        if delta_y <= radius {
            Some(sx-(radius-delta_y)..=sx+(radius-delta_y))
        } else {
            None
        }       
    }).flat_map(|r| {
        r.collect::<Vec<_>>()
    }).collect::<BTreeSet<_>>();
    

    for (_, (bx, by)) in state {
        if by == target_y {
            positions.remove(&bx);
        }
    }
    positions
}

fn main() {
    println!("part1: {}", part1(2_000_000).len());
}
