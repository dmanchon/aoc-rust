use std::collections::HashMap;

fn main() {
    let mut cubes: HashMap<(i32,i32,i32), bool> = HashMap::new();
    for line in include_str!("../input.txt").lines() {
        let p: Vec<_> = line.split(r",").collect();
        cubes.insert((p[0].parse().unwrap(), p[1].parse().unwrap(), p[2].parse().unwrap()), true);
    }

    let mut part1 = 6 * cubes.len() as i32;
    for &(x,y,z) in cubes.keys() {
        for (ix,iy,iz) in [(0,0,1), (0,1,0),(1,0,0),(0,0,-1),(0,-1,0),(-1,0,0)] {
            if cubes.contains_key(&(x+ix,y+iy,z+iz)) {
                part1 -= 1;
            }
        }
    }
 
    println!("part1: {}", part1);
}
