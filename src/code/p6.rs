#![allow(dead_code)]
#![allow(unused_imports)]
use crate::shared;
use itertools::Itertools;

fn read_data() -> &'static str {
    include_str!("../data/p6.txt").trim()
}

fn find_first_start<const WINDOW_SIZE: usize>(s: &str) -> usize {
    'main: for (i, c) in s.as_bytes().array_windows::<WINDOW_SIZE>().enumerate() {
        for (j, ca) in c.iter().enumerate() {
            for (k, cb) in c.iter().enumerate() {
                if j != k && ca == cb {
                    continue 'main;
                }
            }
        }
        return i + WINDOW_SIZE;
    }

    unreachable!()
}

#[test]
fn test() {
    assert_eq!(find_first_start::<4>("asdf_0000"), 4);
    assert_eq!(find_first_start::<4>("asadf_0000"), 5);
    assert_eq!(find_first_start::<4>("asasasfasd_0000"), 11);
}

pub fn a() {
    let d = read_data();
    let ans = find_first_start::<4>(d);
    println!("Part 1: {ans}");
}
pub fn b() {
    let d = read_data();
    let ans = find_first_start::<14>(d);
    println!("Part 2: {ans}");
}
