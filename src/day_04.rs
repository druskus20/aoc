use itertools::{join, Itertools};
use regex::*;

#[derive(Debug)]
pub struct Data {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,

    // Optional
    cid: Option<String>,
}

impl Data {
    fn new(input: &str) -> Data {
        let fields = input
            .split(" ")
            .map(|f| f.split_once(":").unwrap())
            .collect::<Vec<_>>();

        Data {
            cid: fields
                .iter()
                .find(|(k, _)| *k == "cid")
                .map(|(_, b)| b.to_string()),
            pid: fields
                .iter()
                .find(|(k, _)| *k == "pid")
                .map(|(_, b)| b.to_string()),
            ecl: fields
                .iter()
                .find(|(k, _)| *k == "ecl")
                .map(|(_, b)| b.to_string()),
            hcl: fields
                .iter()
                .find(|(k, _)| *k == "hcl")
                .map(|(_, b)| b.to_string()),
            hgt: fields
                .iter()
                .find(|(k, _)| *k == "hgt")
                .map(|(_, b)| b.to_string()),
            eyr: fields
                .iter()
                .find(|(k, _)| *k == "eyr")
                .map(|(_, b)| b.to_string()),
            iyr: fields
                .iter()
                .find(|(k, _)| *k == "iyr")
                .map(|(_, b)| b.to_string()),
            byr: fields
                .iter()
                .find(|(k, _)| *k == "byr")
                .map(|(_, b)| b.to_string()),
        }
    }

    fn is_valid1(&self) -> bool {
        let fields = [
            &self.byr, &self.ecl, &self.eyr, &self.hcl, &self.hgt, &self.iyr, &self.pid,
        ];
        fields.iter().filter(|x| x.is_some()).count() == 7
    }

    // This thing is mad, but its @elkowar's fault.
    fn is_valid2(&self) -> bool {
        self.pid
            .as_ref()
            .map(|x| x.len() == 9 && x.parse::<i32>().is_ok())
            .unwrap_or(false)
            && self
                .byr
                .as_ref()
                .map(|x| {
                    x.parse::<i32>()
                        .map(|x| x >= 1920 && x <= 2002)
                        .unwrap_or(false)
                })
                .unwrap_or(false)
            && self
                .iyr
                .as_ref()
                .map(|x| {
                    x.parse::<i32>()
                        .map(|x| x >= 2010 && x <= 2020)
                        .unwrap_or(false)
                })
                .unwrap_or(false)
            && self
                .eyr
                .as_ref()
                .map(|x| {
                    x.parse::<i32>()
                        .map(|x| x >= 2020 && x <= 2030)
                        .unwrap_or(false)
                })
                .unwrap_or(false)
            && self
                .hgt
                .as_ref()
                .map(|x| {
                    let fuck = x.chars().skip_while(|x| x.is_numeric()).collect::<String>();
                    let num: usize = x.strip_suffix(&fuck).unwrap().parse().unwrap();
                    (fuck == "in" && num >= 59 && num <= 76)
                        || (fuck == "cm" && num >= 150 && num <= 193)
                })
                .unwrap_or(false)
            && self
                .hcl
                .as_ref()
                .map(|x| Regex::new(r"#[a-f0-9]{6}").unwrap().is_match(&*x))
                .unwrap_or(false)
            && self
                .ecl
                .as_ref()
                .map(|x| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&x.as_str()))
                .unwrap_or(false)
    }
}

#[aoc_generator(day4)]
pub fn input_gen(input: &str) -> Vec<Data> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .group_by(|&&l| l.is_empty())
        .into_iter()
        .map(|(_, g)| join(g, " "))
        .filter(|l| !l.is_empty())
        .map(|l| Data::new(&l))
        .collect::<Vec<Data>>()
}

#[aoc(day4, part1)]
pub fn solve_part1(data: &Vec<Data>) -> usize {
    data.iter().filter(|f| f.is_valid1()).count()
}

#[aoc(day4, part2)]
pub fn solve_part2(data: &Vec<Data>) -> usize {
    data.iter().filter(|f| f.is_valid2()).count()
}
