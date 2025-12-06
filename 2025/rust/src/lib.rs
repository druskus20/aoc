extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod prelude {
    pub use color_eyre::Context;
    pub use color_eyre::Result;
    pub use color_eyre::eyre;
    pub use indoc::indoc;
}

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

mod utils;

aoc_lib! { year = 2025 }
