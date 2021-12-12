// Advent of Code 2021 - Day 10

// Might have gone slightly over board with the Chunk methods lol
// Basically solved using a Stack like a decent human being

use std::{collections::HashMap, convert::TryFrom};

use anyhow::{bail, Result};
use itertools::Itertools;

use lazy_static::lazy_static;

type Stack = Vec<Chunk>;
pub type InputType = Vec<String>;
pub type OutputType = i64;

lazy_static! {
    static ref CHUNKS: HashMap<char, char> =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
}

#[aoc_generator(day10)]
pub fn input_gen(input: &str) -> InputType {
    input.lines().map(|l| l.to_owned()).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(data: &InputType) -> Result<OutputType> {
    Ok(data.iter().map(|l| calc_line_error_score(l)).sum())
}

#[aoc(day10, part2)]
pub fn solve_part2(data: &InputType) -> Result<OutputType> {
    let sorted_scores = data
        .iter()
        .filter(|l| calc_line_error_score(l) == 0)
        .map(|l| calc_line_completion_score(l))
        .sorted()
        .collect::<Vec<i64>>();

    Ok(*sorted_scores.get(sorted_scores.len() / 2).unwrap())
}

fn calc_line_error_score(line: &String) -> i64 {
    let mut stack = Stack::new();
    for c in line.chars() {
        let c = Chunk::try_from(c).unwrap();
        if c.is_opening() {
            stack.push(c)
        } else if c.is_closing() {
            let last = stack.pop().unwrap();
            if last.opposite() != c {
                return c.error_score();
            }
        }
    }
    0
}

fn calc_line_completion_score(line: &String) -> i64 {
    let mut stack = Stack::new();
    for c in line.chars() {
        let c = Chunk::try_from(c).unwrap();
        if c.is_opening() {
            stack.push(c);
        } else if c.is_closing() {
            stack.pop().unwrap();
        }
    }

    count_completion_score(&stack)
}

fn count_completion_score(stack: &[Chunk]) -> i64 {
    stack
        .iter()
        .rev()
        .fold(0, |acc, c| (acc * 5) + c.opposite().completion_score())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Chunk {
    OpeningParenthesis,
    OpeningBracket,
    OpeningCurly,
    OpeningAngle,
    ClosingParenthesis,
    ClosingBracket,
    ClosingCurly,
    ClosingAngle,
}

impl TryFrom<char> for Chunk {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '(' => Ok(Chunk::OpeningParenthesis),
            '[' => Ok(Chunk::OpeningBracket),
            '{' => Ok(Chunk::OpeningCurly),
            '<' => Ok(Chunk::OpeningAngle),
            ')' => Ok(Chunk::ClosingParenthesis),
            ']' => Ok(Chunk::ClosingBracket),
            '}' => Ok(Chunk::ClosingCurly),
            '>' => Ok(Chunk::ClosingAngle),
            _ => bail!("Invalid character"),
        }
    }
}

impl ToString for Chunk {
    fn to_string(&self) -> String {
        match self {
            Chunk::OpeningParenthesis => "(".to_string(),
            Chunk::OpeningBracket => "[".to_string(),
            Chunk::OpeningCurly => "{".to_string(),
            Chunk::OpeningAngle => "<".to_string(),
            Chunk::ClosingParenthesis => ")".to_string(),
            Chunk::ClosingBracket => "]".to_string(),
            Chunk::ClosingCurly => "}".to_string(),
            Chunk::ClosingAngle => ">".to_string(),
        }
    }
}

impl Chunk {
    fn opposite(&self) -> Self {
        match self {
            Chunk::OpeningParenthesis => Chunk::ClosingParenthesis,
            Chunk::OpeningBracket => Chunk::ClosingBracket,
            Chunk::OpeningCurly => Chunk::ClosingCurly,
            Chunk::OpeningAngle => Chunk::ClosingAngle,
            Chunk::ClosingParenthesis => Chunk::OpeningParenthesis,
            Chunk::ClosingBracket => Chunk::OpeningBracket,
            Chunk::ClosingCurly => Chunk::OpeningCurly,
            Chunk::ClosingAngle => Chunk::OpeningAngle,
        }
    }

    fn error_score(&self) -> i64 {
        match self {
            Chunk::OpeningParenthesis => 0,
            Chunk::OpeningBracket => 0,
            Chunk::OpeningCurly => 0,
            Chunk::OpeningAngle => 0,
            Chunk::ClosingParenthesis => 3,
            Chunk::ClosingBracket => 57,
            Chunk::ClosingCurly => 1197,
            Chunk::ClosingAngle => 25137,
        }
    }
    fn completion_score(&self) -> i64 {
        match self {
            Chunk::OpeningParenthesis => 1,
            Chunk::OpeningBracket => 1,
            Chunk::OpeningCurly => 1,
            Chunk::OpeningAngle => 1,
            Chunk::ClosingParenthesis => 1,
            Chunk::ClosingBracket => 2,
            Chunk::ClosingCurly => 3,
            Chunk::ClosingAngle => 4,
        }
    }
    fn is_opening(&self) -> bool {
        match self {
            Chunk::OpeningParenthesis
            | Chunk::OpeningBracket
            | Chunk::OpeningCurly
            | Chunk::OpeningAngle => true,
            _ => false,
        }
    }

    fn is_closing(&self) -> bool {
        match self {
            Chunk::ClosingParenthesis
            | Chunk::ClosingBracket
            | Chunk::ClosingCurly
            | Chunk::ClosingAngle => true,
            _ => false,
        }
    }
}
