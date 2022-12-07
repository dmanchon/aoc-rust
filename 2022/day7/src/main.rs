use std::collections::{HashMap, BTreeMap};

use nom::IResult;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::not_line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::character::complete::newline;
use nom::sequence::pair;

#[derive(Debug)]
enum Command<'a> {
    Cd(&'a str),
    Ls(Vec<FileKind<'a>>),
}

#[derive(Debug)]
enum FileKind<'a> {
    File{name: &'a str, size: u32},
    Folder(&'a str),
}

#[derive(Debug, Clone, Copy)]
struct File<'a> {
    name: &'a str,
    size: &'a u32,
}
fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (input, (_, name)) = pair(tag("$ cd "), alt((not_line_ending, tag("/"))))(input)?;
    Ok((input, Command::Cd(name)))
}

fn parse_file(input: &str) -> IResult<&str, FileKind> {
    let (input, (size, name)) = separated_pair(nom::character::complete::u32, tag(" "), not_line_ending)(input)?;
    Ok((input, FileKind::File{size, name}))
}

fn parse_folder(input: &str) -> IResult<&str, FileKind> {
    let (input, (_, name)) = pair(tag("dir "), not_line_ending)(input)?;
    Ok((input, FileKind::Folder(name)))
}

fn parse_dir(input: &str) -> IResult<&str, Vec<FileKind>> {
    let (input, dirs) = separated_list1(newline, alt((parse_file, parse_folder)))(input)?;
    Ok((input, dirs))
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = parse_dir(input)?;
    Ok((input, Command::Ls(files)))
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, cmds) = separated_list1(newline, alt((parse_ls, parse_cd)))(input)?;
    Ok((input, cmds))
}

fn main() {
    let mut cwd: Vec<&str> = vec![];
    let mut fs: HashMap<String, Vec<File>> = HashMap::new();
    
    if let Ok((_, cmds)) = parse_commands(include_str!("../input.txt")) {
	for cmd in &cmds {
	    match cmd {
		Command::Cd("/") => {
		    cwd.push("/");
		},
		Command::Cd("..") => {
		    cwd.pop();
		},
		Command::Cd(path) => {
		    cwd.push(path);
		},
		Command::Ls(paths) => {
		    for path in paths {
			match path {
			    FileKind::File{name, size}=> {
				let file = File{name, size};
				fs.entry(cwd.join("/"))
				    .and_modify(|e| {
					e.push(file);
				    }).or_insert(vec![file]);
			    },
			    f@FileKind::Folder(..) => (),
			}
		    }
		},	 
	    }	 
	}
	let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
	for (path, files) in fs.iter() {
	    let total = files
		.iter()
		.map(|File{size, ..}| *size)
		.sum::<u32>();
	    sizes.insert(path.to_string(), total);
	}
	dbg!(&sizes);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_folder_create() {

    }
}

