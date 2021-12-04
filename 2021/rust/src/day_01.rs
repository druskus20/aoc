// Advent of Code 2021 - Day 1

use anyhow::Result;

#[aoc_generator(day1)]
pub fn input_gen(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(data: &Vec<i32>) -> Result<i32> {
    Ok(data
        .iter()
        .zip(data.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count() as i32)
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &Vec<i32>) -> Result<i32> {
    Ok(data
        .windows(3)
        .zip(data.windows(3).skip(1))
        .filter(|(w1, w2)| w2.iter().sum::<i32>() > w1.iter().sum::<i32>())
        .count() as i32)
}
