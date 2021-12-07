// Advent of Code 2021 - Day 7

// Interesting solution involving finding the minimum of a convex function
// https://github.com/BartoszMilewski/AoC2021/blob/main/Day7.hs

use std::ops::Range;

use anyhow::Result;

type CrabSubmarine = i32;
pub type InputType = Vec<CrabSubmarine>;
pub type OutputType = i32;

#[aoc_generator(day7)]
pub fn input_gen(input: &str) -> InputType {
    input.split(",").map(|line| line.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(data: &InputType) -> Result<OutputType> {
    Ok(tries(data, |nums: &Vec<i32>, n: i32| {
        nums.iter().map(|x| (x - n).abs()).sum()
    }))
}

#[aoc(day7, part2)]
pub fn solve_part2(data: &InputType) -> Result<OutputType> {
    let cost = |n: i32, x: i32| -> i32 {
        let dist = (n - x).abs();
        dist * (dist + 1) / 2
    };
    Ok(tries(data, |nums: &Vec<i32>, n: i32| {
        nums.iter().map(|x| cost(n, *x)).sum()
    }))
}

fn tries(nums: &Vec<i32>, f: impl Fn(&Vec<i32>, i32) -> i32) -> i32 {
    let max: i32 = *nums.iter().max().unwrap();
    let tries = Range::from(0..max)
        .into_iter()
        .map(|n| f(nums, n))
        .collect();
    find_min(tries)
}

// Recursively find the minimum of a convex function
fn find_min(nums: Vec<i32>) -> i32 {
    let mid = nums.len() / 2;
    let left = &nums[..mid];
    let right = &nums[mid + 1..];
    let val = nums[mid];

    go(&left.to_vec(), val, &right.to_vec())
}

fn go(left: &Vec<i32>, v: i32, right: &Vec<i32>) -> i32 {
    match (left.len() == 0, right.len() == 0) {
        (true, true) => v,
        (true, false) => i32::min(v, *right.first().unwrap()),
        (false, true) => i32::min(*left.first().unwrap(), v),
        _ => {
            if *left.last().unwrap() < v {
                find_min(left.to_owned())
            } else if *right.first().unwrap() < v {
                find_min(right.to_owned())
            } else {
                v
            }
        }
    }
}
