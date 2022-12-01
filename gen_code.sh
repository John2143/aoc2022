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

pub fn read_data() -> impl Iterator<Item = i32> {
    include_str!("../data/p$i.txt");
    std::iter::once(0)
}

pub fn a() {}
pub fn b() {}
EOF

    echo "" > "src/data/p$i.txt"
done
