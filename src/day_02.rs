use anyhow::*;
use regex::Regex;
use std::ops::RangeInclusive;

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
            let first_character = *(password.get(first_position).unwrap());
            let sec_position = (l.upper_limit - 1) as usize;
            let sec_character = *(password.get(sec_position).unwrap());

            (first_character != l.letter_required && sec_character == l.letter_required)
                || (first_character == l.letter_required && sec_character != l.letter_required)
        })
        .count()
}

/*
--- Day 2: Password Philosophy ---
Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?

Your puzzle answer was 474.

--- Part Two ---
While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.

Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

1-3 a: abcde is valid: position 1 contains a and position 3 does not.
1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
How many passwords are valid according to the new interpretation of the policies?
 */
