use nom::multi::separated_list1;
use nom::character::complete::newline;
use nom::IResult;
use nom::sequence::separated_pair;
use nom::character::complete::i32 as parse_i32;
use nom::bytes::complete::tag;
use std::collections::BTreeMap;

type Coord = (i32, i32);
type Trace = Vec<Coord>;  

fn trace(s: &str) -> IResult<&str, Trace> {
    separated_list1(
        tag(" -> "),
        separated_pair(parse_i32, tag(","), parse_i32)
    )(s)
}

fn parse(s: &str) -> IResult<&str, Vec<Trace>> {
    separated_list1(newline, trace)(s)
}


fn main() {
    let traces = parse(include_str!("../input.txt")).expect("parsing").1;
    let rocks: Vec<_> = traces.iter().map(
        |trace| trace.windows(2).flat_map(|p| {
            let mut res = vec![];      
            for x in p[0].0.min(p[1].0)..=p[0].0.max(p[1].0) {
                for y in p[0].1.min(p[1].1)..=p[0].1.max(p[1].1) {
                    res.push(((x,y),'#'));
                }
            }
            res
        })
    ).flatten().collect();
    
    let max_y = rocks.iter().map(|r| r.0.1).max().unwrap();
    let mut state = BTreeMap::from_iter(rocks.clone());
    let mut part1 = 0;
    'outer: loop {
        let mut candidate = (500, 0);
        'inner: loop {
            if candidate.1 > max_y {
                break 'outer
            }
            
            let pos_down = (candidate.0, candidate.1+1);
            let pos_down_left = (candidate.0-1, candidate.1+1);
            let pos_down_right = (candidate.0+1, candidate.1+1);

            match (state.get(&pos_down), state.get(&pos_down_left), state.get(&pos_down_right)) {
                (Some(_), Some(_), Some(_)) => {
                    state.insert(candidate, 'o');
                    break 'inner
                },
                (None, _, _) => candidate = pos_down,
                (_, None, _) => candidate = pos_down_left,
                (_, _, None) => candidate = pos_down_right,
            }
        }
        part1 += 1;
    }

    println!("part1: {:#?}", part1);

    let max_y = rocks.iter().map(|r| r.0.1).max().unwrap() + 2;
    let mut state = BTreeMap::from_iter(rocks.clone());

    // floor
    for x in -10000..10000 {
        state.insert((x, max_y), '#');
    }
    
    let mut part2 = 0;
    'outer: loop {
        let mut candidate = (500, 0);
        'inner: loop { 
            let pos_down = (candidate.0, candidate.1+1);
            let pos_down_left = (candidate.0-1, candidate.1+1);
            let pos_down_right = (candidate.0+1, candidate.1+1);

            match (state.get(&pos_down), state.get(&pos_down_left), state.get(&pos_down_right)) {
                (Some(_), Some(_), Some(_)) => {
                    state.insert(candidate, 'o');
                    if candidate == (500, 0) {
                        part2 += 1;
                        break 'outer
                    } else {
                        break 'inner
                    }
                },
                (None, _, _) => candidate = pos_down,
                (_, None, _) => candidate = pos_down_left,
                (_, _, None) => candidate = pos_down_right,
            }
        }
        part2 += 1;
    }
    
    println!("part2: {:#?}", part2);

}
