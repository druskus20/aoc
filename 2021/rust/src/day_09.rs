// Advent of Code 2021 - Day 9

use anyhow::Result;
use std::{collections::HashSet, isize};

pub type InputType = HeightMap;
pub type OutputType = u32;

#[aoc_generator(day9)]
pub fn input_gen(input: &str) -> InputType {
    HeightMap::from(
        input
            .lines()
            .map(|line| line.chars().map(|num| num.to_digit(10).unwrap()).collect())
            .collect::<Vec<Vec<u32>>>(),
    )
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &InputType) -> Result<OutputType> {
    //data.pretty_print();
    let low_points = data.search_lowpoints();
    Ok(low_points
        .iter()
        .map(|(i, j)| data.values[*j][*i] + 1)
        .sum())
}

#[aoc(day9, part2)]
pub fn solve_part2(data: &InputType) -> Result<OutputType> {
    let low_points = data.search_lowpoints();
    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|(x, y)| data.check_basin_size(*x, *y))
        .collect();

    basin_sizes.sort_unstable();

    Ok(basin_sizes.iter().rev().take(3).product::<usize>() as u32)
}

#[derive(Debug, Clone)]
pub struct HeightMap {
    values: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl From<Vec<Vec<u32>>> for HeightMap {
    fn from(values: Vec<Vec<u32>>) -> Self {
        let height = values.len();
        let width = values[0].len();
        Self {
            values,
            height,
            width,
        }
    }
}

impl HeightMap {
    #[allow(dead_code)]
    fn pretty_print(&self) {
        self.values.iter().for_each(|c| {
            c.iter().for_each(|x| print!("{}", x));
            println!();
        });
    }

    fn search_lowpoints(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::new();

        for j in 0..self.height {
            for i in 0..self.width {
                if self.check_low_point(i, j) {
                    points.push((i, j));
                }
            }
        }

        points
    }
    fn check_low_point(&self, i: usize, j: usize) -> bool {
        // Probably better to implement a function that returns an Option
        let range_j = (j as isize - 1).max(0) as usize..(j + 2).min(self.height);
        let range_i = (i as isize - 1).max(0) as usize..(i + 2).min(self.width);

        for new_j in range_j {
            for new_i in range_i.clone() {
                // If we find a lower surrounding point, then (i,j) is the lowest in the cluster
                if self.values[new_j][new_i] < self.values[j][i] {
                    return false;
                }
            }
        }
        true
    }

    fn check_basin_size(&self, x: usize, y: usize) -> usize {
        self.flood(x, y, &mut HashSet::new()).len()
    }

    fn flood<'a>(
        &self,
        i: usize,
        j: usize,
        basin: &'a mut HashSet<(usize, usize)>,
    ) -> &'a mut HashSet<(usize, usize)> {
        if !basin.contains(&(i, j)) {
            basin.insert((i, j));
            let adjacent = [(i - 1, j), (i, j - 1), (i + 1, j), (i, j + 1)];
            for (new_i, new_j) in adjacent {
                if let Some(val) = self.get(new_i, new_j) {
                    if val != 9 {
                        self.flood(new_i, new_j, basin);
                    }
                }
            }
        }
        basin
    }

    fn get(&self, x: usize, y: usize) -> Option<u32> {
        let in_bounds = x < self.width && y < self.height;
        in_bounds.then(|| *self.values.get(y).unwrap().get(x).unwrap())
    }
}
