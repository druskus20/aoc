use anyhow::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::ops::{Range, RangeInclusive};

pub struct LineData {
    lower_limit: u32,
    upper_limit: u32,
    letter_required: char,
    password: String,
}

pub fn parse_line(l: &str) -> Result<LineData> {
    let regex = Regex::new(r"(\d+)-(\d+) (\w): (\w*)").unwrap();
    let res = regex.captures(l).context("Somewhat not working")?;

    Ok(LineData {
        lower_limit: res.get(1).unwrap().as_str().parse()?,
        upper_limit: res.get(2).unwrap().as_str().parse()?,
        letter_required: res.get(3).unwrap().as_str().chars().next().unwrap(),
        password: res.get(4).unwrap().as_str().to_string(),
    })
}

#[aoc_generator(day2)]
pub fn input_gen(input: &str) -> Vec<LineData> {
    input.lines().map(|l| parse_line(l).unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(data: &Vec<LineData>) -> usize {
    let char_appearances = data
        .iter()
        .map(|l| {
            (
                l.password
                    .chars()
                    .filter(|c| *c == l.letter_required)
                    .count(),
                (l.lower_limit..=l.upper_limit),
            )
        })
        .collect::<Vec<(usize, RangeInclusive<u32>)>>();

    // There is probably a better way to do this in the previous iter()
    char_appearances
        .iter()
        .filter(|x| x.1.contains(&(x.0 as u32)))
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(data: &Vec<LineData>) -> usize {
    data.iter()
        .filter(|l| {
            // This is so I keep my sanity somehow
            let password = l.password.chars().collect::<Vec<char>>();
            let first_position = (l.lower_limit - 1) as usize;
            let first_character = (*(password.get(first_position).unwrap()));
            let sec_position = (l.upper_limit - 1) as usize;
            let sec_character = (*(password.get(sec_position).unwrap()));

            (first_character != l.letter_required && sec_character == l.letter_required)
                || (first_character == l.letter_required && sec_character != l.letter_required)
        })
        .count()
}
