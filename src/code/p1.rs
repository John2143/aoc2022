#![allow(dead_code)]
#![allow(unused_imports)]
use crate::shared;
use itertools::Itertools;

#[derive(Debug)]
struct Elf(i32);

pub fn lines() -> impl Iterator<Item = &'static str> {
    include_str!("../data/p1.txt").lines()
}

pub fn read_data() -> impl Iterator<Item = Option<i32>> {
    include_str!("../data/p1.txt")
        .lines()
        .map(|line| match line {
            "" => None,
            n => Some(n.parse::<i32>().unwrap()),
        })
        .chain(std::iter::repeat(None))
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

    elves
}

pub fn a() {
    let elves = get_elves_2();
    let max_cal = elves.map(|elf| elf.0).max().unwrap();
    println!("Part 1: {max_cal}");
}

pub fn b() {
    let mut elves = get_elves_2().collect_vec();

    elves.sort_by_key(|elf| -elf.0);

    let top_three_cal: i32 = elves[0..3].iter().map(|elf| elf.0).sum();
    println!("Part 2: {top_three_cal}");
}

/*
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 */

struct ElfIter<I>(I);

impl<I> Iterator for ElfIter<I>
where
    I: Iterator<Item = Option<i32>>,
{
    type Item = Elf;

    fn next(&mut self) -> Option<Self::Item> {
        let mut tot = 0;
        while let Some(Some(v)) = self.0.next() {
            tot += v;
        }

        match tot {
            0 => None,
            n => Some(Elf(n)),
        }
    }
}

fn get_elves_2() -> impl Iterator<Item = Elf> {
    ElfIter(read_data())
}
