#![allow(dead_code)]
#![allow(unused_imports)]
use std::{
    collections::HashMap,
    ffi::OsString,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::shared::{self, read_number};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while},
    character::{complete::alpha1, is_space},
    combinator::map_res,
    sequence::separated_pair,
    Finish, IResult,
};

fn read_data() -> impl Iterator<Item = &'static str> {
    include_str!("../data/p7.txt").lines()
}

#[derive(Debug, PartialEq)]
enum Command {
    CDDown(OsString),
    LS,
    CDUp,
}

#[derive(Debug, PartialEq)]
enum Line {
    Command(Command),
    DirEnt(OsString),
    File(OsString, usize),
}

fn try_command_input(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("$ ")(input)?;
    let (input, cmd_str) = alpha1(input)?;
    let (input, cmd) = match cmd_str {
        "cd" => {
            let (input, _) = tag(" ")(input)?;
            let (input, dir) = take_till(|_| false)(input)?;
            dbg!(input, dir);
            let cmd = if dir == ".." {
                Command::CDUp
            } else {
                Command::CDDown(dir.into())
            };
            (input, cmd)
        }
        "ls" => (input, Command::LS),
        _ => panic!(),
    };

    Ok((input, Line::Command(cmd)))
}

#[test]
fn cmd_in() {
    assert_eq!(
        try_command_input("$ ls"),
        Ok(("", Line::Command(Command::LS)))
    );

    assert_eq!(
        try_command_input("$ cd .."),
        Ok(("", Line::Command(Command::CDUp)))
    );

    let s = OsString::from("asdf");
    assert_eq!(
        try_command_input("$ cd asdf"),
        Ok(("", Line::Command(Command::CDDown(s))))
    );
}

fn try_dirent(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("dir ")(input)?;
    Ok(("", Line::DirEnt(input.into())))
}

#[test]
fn dirent() {
    assert_eq!(
        try_dirent("dir asdf"),
        Ok(("", Line::DirEnt("asdf".into())))
    );
}

fn read_number_parser<const BASE: u32>(input: &str) -> IResult<&str, usize> {
    read_number(input, BASE)
}

fn filename(c: char) -> bool {
    match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        '.' => true,
        _ => false,
    }
}

fn try_file(input: &str) -> IResult<&str, Line> {
    let (input, size) = read_number_parser::<10>(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, fname) = take_while(filename)(input)?;
    Ok((input, Line::File(fname.into(), size)))
}

#[test]
fn file() {
    assert_eq!(
        try_file("293559 jztrccm.hvd"),
        Ok(("", Line::File("jztrccm.hvd".into(), 293559)))
    );
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((try_command_input, try_dirent, try_file))(input)
}

#[derive(Debug)]
enum Inode {
    Folder(HashMap<OsString, Inode>),
    File(usize),
}

impl Inode {
    fn as_folder(&self) -> &HashMap<OsString, Inode> {
        match self {
            Inode::Folder(f) => f,
            Inode::File(_) => panic!(),
        }
    }

    fn as_folder_mut(&mut self) -> &mut HashMap<OsString, Inode> {
        match self {
            Inode::Folder(f) => f,
            Inode::File(_) => panic!(),
        }
    }

    fn size(&self) -> usize {
        let folder = match self {
            Inode::Folder(f) => f,
            Inode::File(size) => return *size,
        };

        let mut size = 0;
        for (_, inode) in folder {
            size += inode.size();
        }
        size
    }
}

fn navigate(root: &mut Inode, path: impl AsRef<Path>) -> &mut Inode {
    let mut cur_entry = root;
    for p in path.as_ref() {
        cur_entry = cur_entry.as_folder_mut().get_mut(p).unwrap();
    }
    cur_entry
}

fn generate_fs() -> Inode {
    let mut root = Inode::Folder(HashMap::new());
    let mut current_path = PathBuf::new();

    let cmds = read_data()
        .skip(1)
        .map(parse_line)
        .map(|x| x.finish().unwrap().1);

    for cmd in cmds {
        match cmd {
            Line::Command(c) => match c {
                Command::CDDown(p) => current_path.push(p),
                Command::LS => {}
                Command::CDUp => drop(current_path.pop()),
            },
            Line::DirEnt(name) => {
                let dir = navigate(&mut root, &current_path).as_folder_mut();
                dir.insert(name, Inode::Folder(HashMap::new()));
            }
            Line::File(name, size) => {
                let dir = navigate(&mut root, &current_path).as_folder_mut();
                dir.insert(name, Inode::File(size));
            }
        }
    }

    root
}

pub fn a() {
    let ans = generate_fs();
    let mut total = 0;
    for (name, inode) in ans.as_folder() {
        let size = inode.size();
        println!("Inode {name:?} has size {size}");
        if size < 100000 {
            total += size;
        }
    }
    println!("Part 1: {total}");
}
pub fn b() {
    let _d = read_data();
    let ans = "todo";
    println!("Part 2: {ans}");
}
