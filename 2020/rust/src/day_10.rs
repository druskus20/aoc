use core::slice;
use std::{collections::HashSet, iter::FromIterator, slice::Windows};

use anyhow::*;

#[aoc_generator(day9)]
pub fn input_gen(input: &str) -> Vec<i32> {
    parse_input(input)
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &Vec<i32>) -> Result<i32> {
    solve_1(data, 25)
}

#[aoc(day9, part2)]
pub fn solve_part2(data: &Vec<i32>) -> Result<i32> {
    solve_2(data)
}

pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| {
            let num: i32 = l.parse().unwrap();
            num
        })
        .collect::<Vec<i32>>()
}

pub fn solve_1(data: &Vec<i32>, window_size: usize) -> Result<i32> {
}

pub fn solve_2(data: &Vec<i32>) -> Result<i32> {
}

// vim:shiftwidth=4
