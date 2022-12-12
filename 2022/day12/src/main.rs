use petgraph::graphmap::DiGraphMap;
use std::collections::HashMap;
use petgraph::algo::dijkstra;

type Point = (isize, isize);

fn neighbours(point: Point, x_limit: isize, y_limit: isize) -> Vec<Point> {
    let curr_x = point.0;
    let curr_y = point.1;

    let mut result = vec![];
    for (x,y) in [(1,0), (0,1), (-1,0), (0,-1)] {
        let candidate_x = curr_x + x;
        let candidate_y = curr_y + y;

        if candidate_x >= 0 && candidate_x <= x_limit && candidate_y >= 0 && candidate_y <= y_limit {
            result.push((candidate_x, candidate_y));
        }
    }
    result
}
 
fn main() {
    let mut g = DiGraphMap::<(isize, isize, i8), ()>::new();

    let mut x_limit: isize = -1;
    let mut y_limit: isize = -1;
    let mut adj_map: HashMap<Point, i8> = HashMap::new();
    let mut start = None;
    let mut end = None;

    for (y, line) in include_str!("../input.txt").lines().enumerate() {
        let Y: isize = y as isize;
        for (x, node) in line.chars().enumerate() {
            let X: isize = x as isize;
            if node == 'E' {
                end = Some((X, Y, 'z' as i8));
                adj_map.insert((X, Y), 'z' as i8 );
            } else if node == 'S' {
                start = Some((X, Y, 'a' as i8));
                adj_map.insert((X, Y), 'a' as i8);
            } else {
                adj_map.insert((X, Y), node as i8);                
            }
            x_limit = X;
        }
        y_limit = Y;
    }

    for (&point, &w0) in &adj_map {
        for neighbour in neighbours(point, x_limit, y_limit) {
            if let Some(&w1) = adj_map.get(&neighbour){
                let cost =  w1 - w0;
                if cost <= 1 {
                    g.add_edge(
                        (point.0, point.1, w0),
                        (neighbour.0, neighbour.1, w1),
                        ()
                    );
                }
            } 
        }
    }
 
    let res = dijkstra(&g, start.unwrap(), end, |_| 1);
    println!("part1: {}", &res[&end.unwrap()]);

    let part2 = g.nodes().filter(|n| {n.2 == 'a' as i8}).filter_map(|n| {
        let res = dijkstra(&g, n, end, |_| 1);
        res.get(&end.unwrap()).cloned()
    }).min();
    println!("part2: {}", part2.unwrap()); 
}
