// Advent of Code 2021 - Day 9

use anyhow::Result;
use std::{isize, ops::Deref};

pub type InputType = HeightMap;
pub type OutputType = u32;

#[derive(Debug, Clone)]
pub struct HeightMap(Vec<Vec<u32>>);

impl Deref for HeightMap {
    type Target = Vec<Vec<u32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HeightMap {
    fn pretty_print(&self) {
        self.iter().for_each(|c| {
            c.iter().for_each(|x| print!("{}", x));
            println!();
        });
    }

    fn search_lowpoints(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::new();

        for j in 0..self.len() {
            for i in 0..self[0].len() {
                if self.check_low_point(i, j) {
                    points.push((i, j));
                }
            }
        }

        points
    }
    fn check_low_point(&self, i: usize, j: usize) -> bool {
        let range_j = (j as isize - 1).max(0) as usize..(j + 2).min(self.len());
        let range_i = (i as isize - 1).max(0) as usize..(i + 2).min(self[0].len());

        for new_j in range_j {
            for new_i in range_i.clone() {
                // If we find a lower surrounding point, then (i,j) is the lowest in the cluster
                if self[new_j][new_i] < self[j][i] {
                    return false;
                }
            }
        }
        true
    }
}

#[aoc_generator(day9)]
pub fn input_gen(input: &str) -> InputType {
    HeightMap(
        input
            .lines()
            .map(|line| line.chars().map(|num| num.to_digit(10).unwrap()).collect())
            .collect(),
    )
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &InputType) -> Result<OutputType> {
    data.pretty_print();
    let low_points = data.search_lowpoints();

    Ok(low_points.iter().map(|(i, j)| data[*j][*i] + 1).sum())
}

#[aoc(day9, part2)]
pub fn solve_part2(data: &InputType) -> Result<OutputType> {
    todo!()
}
