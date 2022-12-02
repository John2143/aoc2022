#![allow(dead_code)]
#![allow(unused_imports)]
use std::str::FromStr;

use crate::shared;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Strategy(Move, Move);

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,

            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,

            _ => return Err(()),
        })
    }
}

impl Strategy {
    fn get_score(&self) -> usize {
        let base_score = match self.1 {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        let win_score = match (&self.0, &self.1) {
            (Move::Rock, Move::Rock) => 3,
            (Move::Rock, Move::Paper) => 6,
            (Move::Rock, Move::Scissors) => 0,
            (Move::Paper, Move::Rock) => 0,
            (Move::Paper, Move::Paper) => 3,
            (Move::Paper, Move::Scissors) => 6,
            (Move::Scissors, Move::Rock) => 6,
            (Move::Scissors, Move::Paper) => 0,
            (Move::Scissors, Move::Scissors) => 3,
        };

        base_score + win_score
    }
}

fn read_data() -> impl Iterator<Item = Strategy> {
    include_str!("../data/p2.txt")
        .lines()
        .map(|s| Strategy(s[0..1].parse().unwrap(), s[2..3].parse().unwrap()))
}

pub fn a() {
    let s: usize = read_data().map(|s| s.get_score()).sum();

    println!("Part 1: {s}");
}

impl Strategy {
    fn translate_p2(&self) -> Self {
        use Move::*;
        let new_move = match (&self.0, &self.1) {
            (Move::Rock, Move::Rock) => Scissors,
            (Move::Rock, Move::Paper) => Rock,
            (Move::Rock, Move::Scissors) => Paper,
            (Move::Paper, Move::Rock) => Rock,
            (Move::Paper, Move::Paper) => Paper,
            (Move::Paper, Move::Scissors) => Scissors,
            (Move::Scissors, Move::Rock) => Paper,
            (Move::Scissors, Move::Paper) => Scissors,
            (Move::Scissors, Move::Scissors) => Rock,
        };

        Strategy(self.0, new_move)
    }
}

pub fn b() {
    let s: usize = read_data()
        .map(|s| s.translate_p2())
        .map(|s| s.get_score())
        .sum();

    println!("Part 2: {s}");
}
