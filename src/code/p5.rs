#![allow(unused_imports)]
use std::collections::VecDeque;

use crate::shared::{self, read_number};
use itertools::Itertools;
use nom::{
    bytes::complete::{take, take_till},
    character::complete::digit1,
    complete::tag,
    Finish, IResult,
};

pub fn read_data() -> impl Iterator<Item = &'static str> {
    include_str!("../data/p5.txt").lines()
}

struct Stacks {
    boxes: Vec<VecDeque<char>>,
}

fn parse_boxes<'a, I: Iterator<Item = &'a [u8]>>(count: usize, x: I) -> Stacks {
    let mut stacks = Stacks {
        boxes: vec![VecDeque::new(); count],
    };

    for line in x {
        for (tr, box_stack) in line.chunks(4).zip(&mut stacks.boxes) {
            match tr.trim_ascii() {
                b"" => continue,
                [b'[', letter, b']'] => box_stack.push_front(char::from(*letter)),
                _ => unreachable!(),
            }
        }
    }

    stacks
}

#[derive(Debug, PartialEq)]
struct Step {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse_single_move(input: &str) -> IResult<&str, Step> {
    //ex: move 1 from 7 to 4
    let (input, _) = take(5usize)(input)?;
    let (input, amount) = read_number(input, 10)?;
    let (input, _) = take(6usize)(input)?;
    let (input, from) = read_number(input, 10)?;
    let (input, _) = take(4usize)(input)?;
    let (input, to) = read_number(input, 10)?;
    Ok((
        input,
        Step {
            amount,
            from: from - 1,
            to: to - 1,
        },
    ))
}

#[test]
fn asdf() {
    assert_eq!(
        parse_single_move("move 1 from 7 to 4"),
        Ok((
            "",
            Step {
                amount: 1,
                from: 6,
                to: 3,
            }
        ))
    );
}

impl Stacks {
    fn apply_steps_p1<'a, I: Iterator<Item = Step>>(&mut self, moves: I) {
        for step in moves {
            for _i in 0..step.amount {
                let pop = self.boxes[step.from].pop_back().unwrap();
                self.boxes[step.to].push_back(pop);
            }
        }
    }

    fn apply_steps_p2<'a, I: Iterator<Item = Step>>(&mut self, moves: I) {
        for step in moves {
            let from_box = &mut self.boxes[step.from];
            let mut pulled_stack = from_box.split_off(from_box.len() - step.amount);
            self.boxes[step.to].append(&mut pulled_stack);
        }
    }

    //pull the last char from each stack, and then collect them all to a string in order
    fn print_top_stacks(&self) -> String {
        self.boxes.iter().map(|k| k.back().unwrap()).collect()
    }
}

pub fn a() {
    let mut d = read_data();
    let mut boxes = parse_boxes(9, (&mut d).take(8).map(|line| line.as_bytes()));
    boxes.apply_steps_p1(
        d.skip(2)
            .map(parse_single_move)
            .map(Result::unwrap)
            .map(|x| x.1),
    );

    println!("Part 1: {}", boxes.print_top_stacks());
}
pub fn b() {
    let mut d = read_data();
    let mut boxes = parse_boxes(9, (&mut d).take(8).map(|line| line.as_bytes()));
    boxes.apply_steps_p2(
        d.skip(2)
            .map(parse_single_move)
            .map(Result::unwrap)
            .map(|x| x.1),
    );

    println!("Part 2: {}", boxes.print_top_stacks());
}
