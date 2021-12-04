use core::slice;
use std::{collections::HashSet, iter::FromIterator, slice::Windows};

use anyhow::*;

#[aoc_generator(day9)]
pub fn input_gen(input: &str) -> Vec<i64> {
    parse_input(input)
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &Vec<i64>) -> Result<i64> {
    solve_1(data, 25)
}

#[aoc(day9, part2)]
pub fn solve_part2(data: &Vec<i64>) -> Result<i64> {
    solve_2(data)
}

pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| {
            let num: i64 = l.parse().unwrap();
            num
        })
        .collect::<Vec<i64>>()
}

pub fn check_if_valid(num: &i64, previous_numbers: &[i64]) -> bool {
    // num = prev1 + prev2
    let set = HashSet::<i64>::from_iter(previous_numbers.to_vec());
    for e in &set {
        if set.contains(&(num - e)) {
            return true;
        }
    }
    false
}

pub fn solve_1(data: &Vec<i64>, window_size: usize) -> Result<i64> {
    let win = &mut data.windows(window_size + 1);
    let mut weird_number = -1;
    while let Some(slice) = win.next() {
        let previous_n = &slice[..window_size];
        let num = slice.last().unwrap();
        if check_if_valid(num, previous_n) == false {
            weird_number = *num;
            break;
        }
    }
    Ok(weird_number)
}

// Naive Aproach
// Checks the sum of the elements of a slice, begining from the first one.
// If at some point the sum reaches `target`, it stops checking and returns the first
// and last elements of the sum. Otherwise it returns None.
pub fn check_sum(data: &[i64], target: i64) -> Option<(i64, i64)> {
    let mut sum = 0;
    let first = data[0];
    let mut last = -1;
    let mut data = data.iter();

    // This is ugly as fuck but idc
    while sum < target {
        if let Some(e) = data.next() {
            dbg!(e);
            sum += e;
            last = *e;
        } else {
            return None;
        }
    }
    if (sum == target) {
        Some((first, last))
    }
    else {
        None
    }
}

pub fn solve_2(data: &Vec<i64>) -> Result<i64> {
    let number_from_part1 = solve_1(data, 25)?;

    // We can reduce the size of the list by stripping the numbers that are bigger than
    // number_from_part1
    let strip_from: usize = data.iter().position(|e| *e > number_from_part1).unwrap();
    let data = &mut (data[..strip_from].to_vec());

    // We can also remove number_from_part_1
    let number_from_part1_index: Option<usize> = data.iter().position(|e| *e == number_from_part1);
    if let Some(i) = number_from_part1_index {
        data.remove(i);
    }
    
    // BUG: This is not at all what I should do
    let res = data.windows(data.len()-1).find_map(|win| check_sum(win, number_from_part1));
    dbg!(res);
    let res = res.unwrap();
    Ok(res.0 + res.1)
}

// vim:shiftwidth=4
