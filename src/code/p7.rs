#![allow(dead_code)]
#![allow(unused_imports)]
use std::{
    collections::HashMap,
    ffi::OsString,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::shared::{self, read_number, read_number_10};
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

fn filename(c: char) -> bool {
    match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        '.' => true,
        _ => false,
    }
}

fn try_file(input: &str) -> IResult<&str, Line> {
    let (input, (size, fname)) =
        separated_pair(read_number_10, tag(" "), take_while(filename))(input)?;

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

    fn size(&self, running_total: &mut impl FnMut(&OsString, usize) -> ()) -> usize {
        let folder = match self {
            Inode::Folder(f) => f,
            Inode::File(size) => return *size,
        };

        let mut size = 0;
        for (folder_name, inode) in folder {
            let dir_size = inode.size(running_total);
            match inode {
                Inode::Folder(_) => running_total(folder_name, dir_size),
                Inode::File(_) => {}
            }
            size += dir_size;
        }
        //running_total(&OsString::from("tesT"), size);
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
        let size = inode.size(&mut |_, size| {
            if size < 100000 {
                total += size
            }
        });
        println!("Inode {name:?} has size {size}");
    }
    println!("Part 1: {total}");
}
pub fn b() {
    let ans = generate_fs();
    let mut list = Vec::new();
    let total_size = ans.size(&mut |i, size| {
        list.push((i.to_owned(), size));
    });
    list.sort_by_key(|x| x.1);

    let disk_size = 70000000;
    let disk_req = 30000000;
    let current_disk_free = disk_size - total_size;

    for (name, size) in list {
        if current_disk_free + size >= disk_req {
            println!("Part 2: {size}");
            break;
        }
    }
}
