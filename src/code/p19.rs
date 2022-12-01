#![allow(dead_code)]
#![allow(unused_imports)]
use crate::shared;
use itertools::Itertools;

pub fn read_data() -> impl Iterator<Item = i32> {
    include_str!("../data/p19.txt");
    std::iter::once(0)
}

pub fn a() {}
pub fn b() {}
