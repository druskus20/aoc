// Advent of Code 2021 - Day 8

use std::{collections::HashSet, str::FromStr};

use anyhow::Result;
use std::ops::Deref;

pub type InputType = Vec<Entry>;
pub type OutputType = i32;

#[derive(Debug, Clone)]
pub struct Entry {
    signals: Vec<Digit>,
    output: Vec<Digit>,
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('|').collect::<Vec<&str>>();
        let signals = parts[0]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| Digit::from(s))
            .collect();
        let output = parts[1]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| Digit::from(s))
            .collect();
        Ok(Entry { signals, output })
    }
}

#[derive(Debug, Clone)]
pub struct Digit(HashSet<char>);

impl From<&str> for Digit {
    fn from(s: &str) -> Self {
        Digit(s.chars().collect::<HashSet<_>>())
    }
}

impl Deref for Digit {
    type Target = HashSet<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[aoc_generator(day8)]
pub fn input_gen(input: &str) -> InputType {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(data: &InputType) -> Result<OutputType> {
    let count = data
        .iter()
        .flat_map(|e| &e.output)
        .filter(|s| {
            let len = s.0.len();
            len == 2 || len == 3 || len == 4 || len == 7
        })
        .count();

    Ok(count as OutputType)
}

#[aoc(day8, part2)]
pub fn solve_part2(data: &InputType) -> Result<OutputType> {
    Ok(data.iter().map(|e| decode_output(&e)).sum())
}

// Figure out the output by intersecting every digit with one or four
fn decode_output(entry: &Entry) -> i32 {
    let (signals, output) = (&entry.signals, &entry.output);

    // Find the know segments
    let one = signals.iter().find(|s| s.len() == 2).unwrap();
    let four = signals.iter().find(|s| s.len() == 4).unwrap();

    output.iter().fold(0i32, |num_acc, digit| {
        let digit = match (
            digit.len(),
            one.intersection(&digit).count(),
            four.intersection(&digit).count(),
        ) {
            (2, _, _) => 1,
            (3, _, _) => 7,
            (4, _, _) => 4,
            (5, 2, _) => 3,
            (5, _, 2) => 2,
            (5, _, _) => 5,
            (6, 1, _) => 6,
            (6, _, 4) => 9,
            (6, _, _) => 0,
            (7, _, _) => 8,
            _ => unreachable!(),
        };
        (num_acc * 10) + digit
    })
}
