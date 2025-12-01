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
mod utils;

aoc_lib! { year = 2025 }
