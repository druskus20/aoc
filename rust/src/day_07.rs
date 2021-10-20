use anyhow::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Rules = HashMap<String, HashSet<String>>;
type NumRules = HashMap<String, HashMap<String, u32>>;

lazy_static! {
    static ref ORIGIN_REG: Regex = Regex::new(r"^\w* \w*").unwrap();
    static ref DESTINATIONS_REG: Regex = Regex::new(r"(\d) (\w* \w+) bags*[,. ]*").unwrap();
}

#[aoc_generator(day7, part1)]
pub fn input_gen1(input: &str) -> Rules {
    input.lines().map(parse_line1).collect::<Rules>()
}

pub fn parse_line1(line: &str) -> (String, HashSet<String>) {
    let origin = ORIGIN_REG
        .captures(line)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_string();

    let destinations = DESTINATIONS_REG
        .captures_iter(line)
        .map(|c| c.get(2).unwrap().as_str().to_string())
        .collect::<HashSet<String>>();

    (origin, destinations)
}

#[aoc(day7, part1)]
pub fn solve_part1(data: &Rules) -> usize {
    get_bags_that_contain("shiny gold".parse().unwrap(), data).len() - 1 // -1 bc the shiny gold doesnt count
}

// This could be easily adapted to count all specific combinations of bags instead of just different colors
//  by returning the length of get_bags_that_contain + 1 instead.
pub fn get_bags_that_contain(target: String, rules: &Rules) -> HashSet<String> {
    let mut colors = rules
        .iter()
        .filter(|(_, smallbags)| smallbags.contains(&*target))
        .map(|(bigbag, _)| get_bags_that_contain(bigbag.to_string(), rules))
        .flatten()
        .collect::<HashSet<String>>();

    colors.insert(target);
    colors
}

#[aoc_generator(day7, part2)]
pub fn input_gen2(input: &str) -> NumRules {
    input.lines().map(parse_line2).collect::<NumRules>()
}
pub fn parse_line2(line: &str) -> (String, HashMap<String, u32>) {
    let origin = ORIGIN_REG
        .captures(line)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_string();

    let destinations = DESTINATIONS_REG
        .captures_iter(line)
        .map(|c| {
            (
                c.get(2).unwrap().as_str().to_string(),
                c.get(1).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect::<HashMap<String, u32>>();

    (origin, destinations)
}

#[aoc(day7, part2)]
pub fn solve_part2(data: &NumRules) -> usize {
    (get_num_bags_that_contain("shiny gold".parse().unwrap(), data)) as usize
    // -1 bc the shiny gold doesnt count
}

pub fn get_num_bags_that_contain(target: String, rules: &NumRules) -> u32 {
    rules
        .iter()
        .filter(|(t, _)| **t == target)
        .map(|(_, sub_bags)| {
            sub_bags
                .iter()
                .map(|(name, num)| num * (get_num_bags_that_contain(name.clone(), rules) + 1))
                .sum::<u32>()
        })
        .sum()
}
