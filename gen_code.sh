#!/bin/bash

if [ -z "$1" ];
then
    echo "supply minimum num to generate";
    exit 1;
fi

for i in $(seq $1 25)
do
    cat <<EOF > "src/code/p$i.rs"
#![allow(dead_code)]
#![allow(unused_imports)]
use crate::shared;
use itertools::Itertools;

fn read_data() -> impl Iterator<Item = &'static str> {
    include_str!("../data/p$i.txt").lines()
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
EOF

    echo "" > "src/data/p$i.txt"
done
