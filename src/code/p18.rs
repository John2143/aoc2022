#![allow(dead_code)]
#![allow(unused_imports)]
use crate::shared;
use itertools::Itertools;

fn read_data() -> impl Iterator<Item = &'static str> {
    include_str!("../data/p18.txt").lines()
}

pub fn a() {
    let _d = read_data();
    let ans = "todo";
    println!("Part 1: {ans}");
}
pub fn b() {
    let _d = read_data();
    let ans = "todo";
    println!("Part 2: {ans}");
}
