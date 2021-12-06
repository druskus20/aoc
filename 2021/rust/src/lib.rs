#![feature(drain_filter)]
#![feature(generators, generator_trait)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

aoc_lib! { year = 2021 }
