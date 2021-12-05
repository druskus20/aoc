// Advent of Code 2021 - Day 3

use anyhow::{anyhow, Result};

pub type InputType = (Vec<i32>, i32);
pub type OutputType = i32;

#[aoc_generator(day3)]
pub fn input_gen(input: &str) -> InputType {
    let data = input
        .lines()
        .map(|l| i32::from_str_radix(l, 2).unwrap())
        .collect();

    let lines = input.lines().collect::<Vec<&str>>();
    (data, lines.get(0).unwrap().len() as i32)
}

fn get_bit_at(n: i32, position: i32) -> i32 {
    (n & (1 << position)) >> position
}

fn set_bit_at(n: &mut i32, position: i32, val: bool) {
    if val {
        *n = *n | (1 << position)
    } else {
        *n = *n & !(1 << position)
    }
}

fn limit_bit_length(n: i32, length: i32) -> i32 {
    let mask = !(i32::MAX << length);
    n & mask
}

fn get_most_common_value_at_position(nums: &Vec<i32>, pos: i32) -> bool {
    let half_total = nums.len() as f32 / 2.0;
    let total_ones = nums.iter().map(|n| get_bit_at(*n, pos)).sum::<i32>();
    (total_ones as f32) >= half_total
}

fn calc_gamma_rate(nums: &Vec<i32>, len: i32) -> i32 {
    let mut mask = 0b01;
    let mut gamma_rate = 0;

    for pos in 0..len {
        let val = get_most_common_value_at_position(nums, pos);
        set_bit_at(&mut gamma_rate, pos, val);
        mask = mask << 1;
    }

    gamma_rate
}

#[aoc(day3, part1)]
pub fn solve_part1((data, len): &InputType) -> Result<OutputType> {
    let gamma_rate = calc_gamma_rate(data, *len);

    // Calculate epsilon from gamma
    let epsilon_rate = limit_bit_length(!gamma_rate, *len);

    Ok(gamma_rate * epsilon_rate)
}

// For each bit, for each number, filters the vector based on a criteria until
// there's only one number left.
fn filter_bits_by_criteria(
    data: &Vec<i32>,
    len: i32,
    criteria: impl Fn(&Vec<i32>, i32, i32) -> bool,
) -> Result<i32> {
    let filtered_data = &mut data.clone();

    for pos in (0..len).rev() {
        *filtered_data = filtered_data
            .iter()
            .filter(|n| criteria(&filtered_data, **n, pos))
            .cloned()
            .collect::<Vec<i32>>();
        if filtered_data.len() == 1 {
            return Ok(*filtered_data.get(0).unwrap());
        }
    }

    Err(anyhow!("No single value found"))
}

#[aoc(day3, part2)]
pub fn solve_part2((data, len): &InputType) -> Result<OutputType> {
    let oxigen_generator_rating = filter_bits_by_criteria(data, *len, |data, num, pos| {
        let most_common_val = get_most_common_value_at_position(data, pos) as i32;
        get_bit_at(num, pos) == most_common_val
    })?;
    let co2_scrubber_rating = filter_bits_by_criteria(data, *len, |data, num, pos| {
        get_bit_at(num, pos) == (!get_most_common_value_at_position(data, pos)) as i32
    })?;

    Ok(oxigen_generator_rating * co2_scrubber_rating)
}
