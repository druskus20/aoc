// Advent of Code 2021 - Day 6

// The naive approach works for the first part, but it's unneficient and that is reflected in the
// second part.

use anyhow::Result;

type Lanternfish = usize;
pub type InputType = Vec<Lanternfish>;
pub type OutputType = i64;

#[aoc_generator(day6)]
pub fn input_gen(input: &str) -> InputType {
    input.split(",").map(|n| n.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(data: &InputType) -> Result<OutputType> {
    Ok(calc_lanterfish_after(data, 80))
}

#[aoc(day6, part2)]
pub fn solve_part2(data: &InputType) -> Result<OutputType> {
    Ok(calc_lanterfish_after(data, 256))
}

fn calc_lanterfish_after(fish: &InputType, days: i32) -> i64 {
    let mut counts = [0i64; 9];
    fish.iter().for_each(|v| counts[*v] += 1);
    for _ in 0..days {
        counts[7] += counts[0]; // every fish that reaches 0 spawns a new one
        counts.rotate_left(1);
    }
    counts.iter().sum()
}

/*
// Naive aproach
fn calc_lanterfish_after(data: &InputType, days: i32) -> i32 {
    let mut data = data.clone();
    for _ in 0..days {
        data = data
            .iter()
            .flat_map(|n| match n {
                0 => vec![6, 8],
                _ => vec![n - 1],
            })
            .collect();
    }
    data.iter().count() as i32
}
*/
