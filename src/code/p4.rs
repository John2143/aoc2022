#![allow(dead_code)]
#![allow(unused_imports)]
use std::{ops::RangeInclusive, str::FromStr};

use crate::shared::{self, read_number};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

struct Assignment(RangeInclusive<usize>, RangeInclusive<usize>);

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<usize>> {
    let (input, range_bot) = read_number(input, 10)?;
    let (input, _) = tag("-")(input)?;
    let (input, range_top) = read_number(input, 10)?;
    Ok((input, range_bot..=range_top))
}

impl FromStr for Assignment {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        fn parse(input: &str) -> IResult<&str, Assignment> {
            let (input, (r1, r2)) = separated_pair(parse_range, tag(","), parse_range)(input)?;
            Ok((input, Assignment(r1, r2)))
        }
        Ok(parse(input).unwrap().1)
    }
}

fn read_data() -> impl Iterator<Item = Assignment> {
    include_str!("../data/p4.txt")
        .lines()
        .map(|x| x.parse().unwrap())
}

#[test]
fn test() {
    assert_eq!(parse_range("123-456"), Ok(("", 123..=456)));
    assert_eq!(parse_range("123-456,456-678"), Ok((",456-678", 123..=456)));
}

impl Assignment {
    fn range_includes_p1(&self) -> bool {
        if self.0.start() <= self.1.start() && self.0.end() >= self.1.end() {
            return true;
        }
        if self.1.start() <= self.0.start() && self.1.end() >= self.0.end() {
            return true;
        }
        false
    }
    fn range_includes_p2(&self) -> bool {
        if self.0.end() >= self.1.start() && self.0.end() <= self.1.end() {
            return true;
        }
        if self.1.end() >= self.0.start() && self.1.end() <= self.0.end() {
            return true;
        }
        false
    }
}

pub fn a() {
    let amount_overlapping = read_data().filter(|x| x.range_includes_p1()).count();
    println!("Part 1: {amount_overlapping}");
}
pub fn b() {
    let amount_overlapping = read_data().filter(|x| x.range_includes_p2()).count();
    println!("Part 2: {amount_overlapping}");
}
