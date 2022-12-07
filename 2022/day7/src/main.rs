use std::collections::HashMap;

// in general terms is just same as python, not idiomatic, try later to refactor
fn main() {
    let mut fs: HashMap<String, usize> = HashMap::new();
    let lines: Vec<_> = include_str!("../input.txt").lines().collect();
    
    let mut i = 1;
    let mut current: Vec<String>  = vec!["".to_string()];
    let mut dirs : Vec<String> = vec!["/".to_string()];

    while i < lines.len() {        
        if lines[i].starts_with(r"$ ls") {
            loop {
                i += 1;
                if i == lines.len() {break;}
                let res: Vec<_> = lines[i].split(r" ").collect();

                match res[0] {
                    "$"|"" => {break;}
                    "dir" => {
                        let mut tmp = current.clone();
                        tmp.push(res[1].to_string());
                        let s = format!("{}/", tmp.join("/"));
                        dirs.push(s);   
                    },
                    size => {
                        // this is ugly: refactor -> build_path?
                        let mut tmp = current.clone();
                        tmp.push(res[1].to_string());
                        let path = tmp.join("/");
                        *fs.entry(path.to_string()).or_insert(0) += size.parse::<usize>().unwrap();
                    }
                }
            }
            continue;
        } else if lines[i].starts_with(r"$ cd "){
            let dir: Vec<_> = lines[i].split(r"$ cd ").collect();
            if dir[1] == ".." {
                current.pop();
            } else {
                current.push(dir[1].to_string());
            }
        }
        i += 1;
    }

    let mut sizes: HashMap<&String, usize> = HashMap::new();
    for dir in &dirs {
        let a: usize = fs.iter().filter(|(k,_)|k.starts_with(dir)).map(|(_, &v)| v).sum();
        sizes.insert(dir, a);
    }

    let part1 = sizes.clone().into_values().filter(|&x| x<100000).sum::<usize>();
    println!("part1: {}", part1);

    let required = 30000000 - (70000000 - sizes[&"/".to_string()]);
    let mut values = sizes.into_values().collect::<Vec<_>>();
    values.sort();
    let part2 = &values.iter().find(|&&x| x > required);
    println!("part2: {:#?}", part2.unwrap());
}
