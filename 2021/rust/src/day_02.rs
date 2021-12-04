// Advent of Code 2021 - Day 2

use std::str::FromStr;

use anyhow::Result;
use scan_fmt::scan_fmt;

pub type InputType = Vec<(Command, i32)>;
pub type OutputType = i32;

pub enum Command {
    Up,
    Down,
    Forward,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Command::Up),
            "down" => Ok(Command::Down),
            "forward" => Ok(Command::Forward),
            _ => Err(anyhow::anyhow!("Invalid direction")),
        }
    }
}

struct Submarine {
    depth: i32,
    position: i32,
    aim: i32,
}

impl Default for Submarine {
    fn default() -> Self {
        Self {
            depth: 0,
            position: 0,
            aim: 0,
        }
    }
}

impl Submarine {
    fn apply_command(&mut self, c: &(Command, i32)) {
        match c {
            (Command::Up, x) => self.depth -= x,
            (Command::Down, x) => self.depth += x,
            (Command::Forward, x) => self.position += x,
        }
    }

    fn apply_complicated_command(&mut self, c: &(Command, i32)) {
        match c {
            (Command::Up, x) => self.aim -= x,
            (Command::Down, x) => self.aim += x,
            (Command::Forward, x) => {
                self.position += x;
                self.depth += self.aim * x;
            }
        }
    }
}

#[aoc_generator(day2)]
pub fn input_gen(input: &str) -> InputType {
    input
        .lines()
        .map(|l| {
            let (s, n) = scan_fmt!(l, "{} {d}", String, i32).unwrap();
            (Command::from_str(&s).unwrap(), n)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(data: &InputType) -> Result<OutputType> {
    let mut s = Submarine::default();
    data.iter().for_each(|c| s.apply_command(c));
    Ok(s.depth * s.position)
}

#[aoc(day2, part2)]
pub fn solve_part2(data: &InputType) -> Result<OutputType> {
    let mut s = Submarine::default();
    data.iter().for_each(|c| s.apply_complicated_command(c));
    Ok(s.depth * s.position)
}
