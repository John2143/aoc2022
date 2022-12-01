#![allow(dead_code)]
#![allow(unused_imports)]
use crate::shared;
use itertools::Itertools;

#[derive(Debug)]
struct Elf(i32);

pub fn read_data() -> impl Iterator<Item = Option<i32>> {
    include_str!("../data/p1.txt").lines().map(|x| match x {
        "" => None,
        x => Some(x.parse::<i32>().unwrap()),
    })
}

fn get_elves() -> Vec<Elf> {
    let iter = read_data();
    let mut cur_elf = Elf(0);
    let mut elves = vec![];

    for val in iter {
        match val {
            Some(n) => cur_elf.0 += n,
            None => {
                elves.push(cur_elf);
                cur_elf = Elf(0);
            }
        }
    }

    elves.push(cur_elf);

    elves
}

pub fn a() {
    let elves = get_elves();
    let max_cal = elves.into_iter().map(|elf| elf.0).max().unwrap();
    println!("Max calories: {max_cal}");
}

pub fn b() {
    let mut elves = get_elves();

    elves.sort_by_key(|elf| elf.0);

    let top_three = &elves[(elves.len() - 3)..];
    let top_three_cal: i32 = top_three.iter().map(|elf| elf.0).sum();
    println!("Max calories top three: {top_three_cal}");
}
