use itertools::Itertools;

/// Advent of Code 2025 - Day 1
///
/// There's definitely a way to do this with math, something like
///
/// for each p_digits:
/// n = (p) || (p)
/// n = (p * 10^p_digits) + (p)
///
/// I can't figure it out right now, so im using regex
use crate::{prelude::*, utils};

const USE_MOCK: bool = false;
const MOCK_DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

type Input = Vec<(i64, i64)>;

#[aoc_generator(day2)]
pub fn input_gen(input: &str) -> Input {
    let input = if USE_MOCK {
        eprintln!("------------------");
        eprintln!("Using mock data!");
        eprintln!("------------------");
        MOCK_DATA
    } else {
        input
    };

    input
        .split(',')
        .map(|s| {
            let index = s.find('-').unwrap();
            let (s1, s2) = s.split_at(index);
            (s1.parse().unwrap(), s2[1..].parse().unwrap())
        })
        .collect()
}

fn filter_patterns_by_regex(regex: &str, data: &Input) -> Vec<i64> {
    let regex = fancy_regex::Regex::new(regex).unwrap();
    let mut matching_patterns = Vec::new();

    for (p_start, p_end) in data.iter() {
        for p in *p_start..=*p_end {
            let p_str = p.to_string();
            if regex.is_match(&p_str).unwrap() {
                matching_patterns.push(p);
            }
        }
    }

    matching_patterns
}

#[aoc(day2, part1)]
pub fn solve_part1(data: &Input) -> Result<i64> {
    let regex: &str = "^([1-9]\\d*)\\1$";
    let matching_patterns = filter_patterns_by_regex(regex, data);
    let c = matching_patterns.iter().unique().sum::<i64>();
    Ok(c)
}

#[aoc(day2, part2)]
pub fn solve_part2(data: &Input) -> Result<i64> {
    let regex: &str = "^([1-9]\\d*)\\1+$";
    let matching_patterns = filter_patterns_by_regex(regex, data);
    let c = matching_patterns.iter().unique().sum::<i64>();
    Ok(c)
}
