#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::HashSet;

use crate::shared;
use itertools::Itertools;

#[derive(Debug)]
struct Rucksack<'a>(&'a [u8], &'a [u8]);

fn priority(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 1 + 26,
        _ => unreachable!(),
    }
}

#[test]
fn asdfasdf() {
    assert_eq!(priority(b'a'), 1);
    assert_eq!(priority(b'z'), 26);
    assert_eq!(priority(b'A'), 27);
    assert_eq!(priority(b'Z'), 52);
}

fn read_data() -> impl Iterator<Item = Rucksack<'static>> {
    include_str!("../data/p3.txt").lines().map(|line| {
        let line = line.as_bytes();
        let split = line.len() / 2;
        let (left, right) = line.split_at(split);
        Rucksack(left, right)
    })
}

impl Rucksack<'_> {
    fn find_match(&self) -> u8 {
        for l_c in self.0 {
            for r_c in self.1 {
                if l_c == r_c {
                    return *l_c;
                }
            }
        }

        panic!("no match: {:?} {:?}", self.0, self.1);
    }
}

pub fn a() {
    let prio_sum: usize = read_data().map(|x| priority(x.find_match()) as usize).sum();
    println!("Part 1: {}", prio_sum);
}
pub fn b() {
    let prio_sum: usize = read_data()
        .array_chunks::<3>()
        .map(|chunks| {
            let sets: [HashSet<u8>; 3] = chunks.map(|sack| {
                HashSet::from_iter(sack.0.iter().copied().chain(sack.1.iter().copied()))
            });
            let int = sets[0].intersection(&sets[1]);
            let int_map = HashSet::from_iter(int.copied());

            let mut all_match = int_map.intersection(&sets[2]);
            priority(*all_match.next().unwrap()) as usize
        })
        .sum();

    println!("Part 2: {}", prio_sum);
}
