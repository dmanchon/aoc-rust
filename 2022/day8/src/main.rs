use ansi_term::Colour::RGB;
use std::fmt;



struct Map {
    size: usize,
    cells: Vec<Vec<i16>>,
}

#[derive(Debug)]
enum Dir {
    Right,
    Left,
    Down,
    Up,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "\n");
        for i in 0..self.size {
	    for j in 0..self.size {
                let val = 240-(20*self.index(i, j));
	        let _ = write!(f, "{}", RGB(255, val, val).paint(format!("{}", self.index(i, j))));
	    }
	    let _ = write!(f, "\n");
        }
        fmt::Result::Ok(())
    }
}

impl Map {
    fn new(data: &[u8]) -> Map {
	let mut cells: Vec<Vec<i16>> = vec![];
	for row in data.split(|&c| c == b'\n') {
	    cells.push(row
		       .into_iter()
		       .map(|&c| c as char)
		       .filter_map(|c| c.to_digit(10))
		       .map(|d| d as i16)
		       .collect());
	}
	let size = cells[0].len();
	Map{cells, size}
    }

    fn index(&self, x: usize, y: usize) -> u8 {
	self.cells[x][y] as u8
    }
    fn index2(&self, x: isize, y: isize) -> u8 {
	self.cells[x as usize][y as usize] as u8
    }

    fn part1(&self)->i32 {
        let mut state = vec![vec![false; self.size]; self.size];
        
        for i in 0..self.size {
            for j in 0..self.size {
                if j == 0 || j == self.size - 1 || i == 0 || i == self.size - 1 {
                    state[i][j] = true;
                }
            }
        }
        
        //all rows and columns
        for i in 0..self.size {
            let mut current = 0;
            for j in 0..self.size {
                if i == 0 {
                    current = self.index(i, j);
                } else if self.index(i, j) > current {
                    current = self.index(i, j);
                    state[i][j] = true;
                }
            }
        }
        for i in (0..self.size).rev() {
            let mut current = 0;
            for j in (0..self.size).rev() {
                if j == self.size - 1 {
                    current = self.index(i, j);
                } else if self.index(i, j) > current {
                    current = self.index(i, j);
                    state[i][j] = true;
                }
            }
        }
        for j in 0..self.size {
            let mut current = 0;
            for i in 0..self.size {
                if i == 0 {
                    current = self.index(i, j);
                } else if self.index(i, j) > current {
                    current = self.index(i, j);
                    state[i][j] = true;
                }
            }
        }
        for j in (0..self.size).rev() {
            let mut current = 0;
            for i in (0..self.size).rev() {
                if j == self.size - 1 {
                    current = self.index(i, j);
                } else if self.index(i, j) > current {
                    current = self.index(i, j);
                    state[i][j] = true;
                }
            }
        }

        let mut count = 0;
        for r in &state {
            for &e in r {
                if e {
                    count += 1;
                } 
            }
        }
        count
    }

    fn helper(&self, x0: isize, y0: isize, x1: isize, y1: isize, dir: Dir, acc: u32) -> u32 {
        //dbg!(x0,y0,x1,y1,acc,&dir,self.index2(x0, y0));
        let len: isize = self.size as isize - 1;
        if x1 >= 0  && x1 <= len && y1 >= 0 && y1 <= len {
            match dir {
                Dir::Down => {
                    //dbg!(self.index2(x1, y1));
                    if self.index2(x0, y0) == self.index2(x1, y1) {
                        acc+1
                    } else if self.index2(x0, y0) > self.index2(x1, y1) {
                        self.helper(x0, y0, x1+1, y1, dir, acc+1)
                    } else {
                        acc+1
                    }
                },
                Dir::Up => {
                    if self.index2(x0, y0) == self.index2(x1, y1) {
                        acc+1
                    } else if self.index2(x0, y0) > self.index2(x1, y1) {
                        self.helper(x0, y0, x1-1, y1, dir, acc+1)
                    } else {
                        acc+1
                    }
                },
                Dir::Right => {
                    if self.index2(x0, y0) == self.index2(x1, y1) {
                        acc+1
                    } else if self.index2(x0, y0) > self.index2(x1, y1) {
                        self.helper(x0, y0, x1, y1+1, dir, acc+1)
                    } else {
                        acc+1
                    }
                },
                Dir::Left => {
                    if self.index2(x0, y0) == self.index2(x1, y1) {
                        acc+1
                    } else if self.index2(x0, y0) > self.index2(x1, y1) {
                        self.helper(x0, y0, x1, y1-1, dir, acc+1)
                    } else {
                        acc+1
                    }
                },
            }
        } else {
            acc
        }
    }
    fn visibility(&self, x: isize, y: isize) -> [u32;4] {
        
        [
            self.helper(x, y, x+1, y, Dir::Down, 0),
            self.helper(x, y, x-1, y, Dir::Up, 0),
            self.helper(x, y, x, y+1, Dir::Right, 0),
            self.helper(x, y, x, y-1, Dir::Left, 0)
        ]
            
    }

    fn part2(&self) -> u32 {
        let mut max = 0;

        for i in 0..self.size {
            for j in 0..self.size {
                let [s0,s1,s2,s3] = self.visibility(i as isize, j as isize);
                let score = s0*s1*s2*s3;
                if score > max {
                    max = score;
                }
            }
        }

        max
    }
}

fn main() {
    let map = Map::new(include_bytes!("../input.txt"));
    dbg!(&map);

    println!("part1: {}", &map.part1());
    println!("part2: {}", &map.part2());
}
