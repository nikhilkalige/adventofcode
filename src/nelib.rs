#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
pub mod day_01 {
    use std::collections::HashSet;
    pub fn input_generator(input: &str) -> HashSet<u32> {
        input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.parse().unwrap())
            .collect()
    }
    #[doc(hidden)]
    pub mod __input_generator_aoc_generator {
        use super::*;
        impl<'a> aoc_runner::Generator<'a> for crate::__aoc::Day1Part1Alt0Generator {
            type Output = HashSet<u32>;
            fn generate(
                &self,
                input: &'a str,
            ) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                Ok(input_generator(input))
            }
        }
        impl<'a> aoc_runner::Generator<'a> for crate::__aoc::Day1Part1Alt1Generator {
            type Output = HashSet<u32>;
            fn generate(
                &self,
                input: &'a str,
            ) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                Ok(input_generator(input))
            }
        }
        impl<'a> aoc_runner::Generator<'a> for crate::__aoc::Day1Part1Alt2Generator {
            type Output = HashSet<u32>;
            fn generate(
                &self,
                input: &'a str,
            ) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                Ok(input_generator(input))
            }
        }
        impl<'a> aoc_runner::Generator<'a> for crate::__aoc::Day1Part1Alt3Generator {
            type Output = HashSet<u32>;
            fn generate(
                &self,
                input: &'a str,
            ) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                Ok(input_generator(input))
            }
        }
        impl<'a> aoc_runner::Generator<'a> for crate::__aoc::Day1Part1Alt4Generator {
            type Output = HashSet<u32>;
            fn generate(
                &self,
                input: &'a str,
            ) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                Ok(input_generator(input))
            }
        }
        impl<'a> aoc_runner::Generator<'a> for crate::__aoc::Day1Part2Alt0Generator {
            type Output = HashSet<u32>;
            fn generate(
                &self,
                input: &'a str,
            ) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                Ok(input_generator(input))
            }
        }
        impl<'a> aoc_runner::Generator<'a> for crate::__aoc::Day1Part2Alt1Generator {
            type Output = HashSet<u32>;
            fn generate(
                &self,
                input: &'a str,
            ) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                Ok(input_generator(input))
            }
        }
        impl<'a> aoc_runner::Generator<'a> for crate::__aoc::Day1Part2Alt2Generator {
            type Output = HashSet<u32>;
            fn generate(
                &self,
                input: &'a str,
            ) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                Ok(input_generator(input))
            }
        }
        impl<'a> aoc_runner::Generator<'a> for crate::__aoc::Day1Part2Alt3Generator {
            type Output = HashSet<u32>;
            fn generate(
                &self,
                input: &'a str,
            ) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                Ok(input_generator(input))
            }
        }
        impl<'a> aoc_runner::Generator<'a> for crate::__aoc::Day1Part2Alt4Generator {
            type Output = HashSet<u32>;
            fn generate(
                &self,
                input: &'a str,
            ) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                Ok(input_generator(input))
            }
        }
    }
    pub fn solve_part_01(input: &HashSet<u32>) -> u32 {
        for x in input {
            let y = 2020 - x;
            if input.contains(&y) {
                return x * y;
            }
        }
        0
    }
    #[doc(hidden)]
    mod __solve_part_01_runner {
        use super::*;
        use crate::__aoc::Day1Part1Alt0Runner;
        use aoc_runner::Runner;
        impl<'a> Runner<'a, &'a HashSet<u32>> for Day1Part1Alt0Runner<&'a HashSet<u32>> {
            type Output = u32;
            fn run(
                &self,
                input: &'a HashSet<u32>,
            ) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                Ok(solve_part_01(input))
            }
        }
    }
}
#[doc(hidden)]
pub mod __aoc {
    use aoc_runner::{Generator, GeneratorDefault, Runner, RunnerDefault};
    use std::marker::PhantomData;
    pub const YEAR: u32 = 2020u32;
    pub struct Day1Part1Alt0Generator;
    impl GeneratorDefault for Day1Part1Alt0Generator {}
    pub struct Day1Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day1Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day1Part1Alt1Generator;
    impl GeneratorDefault for Day1Part1Alt1Generator {}
    pub struct Day1Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day1Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day1Part1Alt2Generator;
    impl GeneratorDefault for Day1Part1Alt2Generator {}
    pub struct Day1Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day1Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day1Part1Alt3Generator;
    impl GeneratorDefault for Day1Part1Alt3Generator {}
    pub struct Day1Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day1Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day1Part1Alt4Generator;
    impl GeneratorDefault for Day1Part1Alt4Generator {}
    pub struct Day1Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day1Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day1Part2Alt0Generator;
    impl GeneratorDefault for Day1Part2Alt0Generator {}
    pub struct Day1Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day1Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day1Part2Alt1Generator;
    impl GeneratorDefault for Day1Part2Alt1Generator {}
    pub struct Day1Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day1Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day1Part2Alt2Generator;
    impl GeneratorDefault for Day1Part2Alt2Generator {}
    pub struct Day1Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day1Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day1Part2Alt3Generator;
    impl GeneratorDefault for Day1Part2Alt3Generator {}
    pub struct Day1Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day1Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day1Part2Alt4Generator;
    impl GeneratorDefault for Day1Part2Alt4Generator {}
    pub struct Day1Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day1Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day2Part1Alt0Generator;
    impl GeneratorDefault for Day2Part1Alt0Generator {}
    pub struct Day2Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day2Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day2Part1Alt1Generator;
    impl GeneratorDefault for Day2Part1Alt1Generator {}
    pub struct Day2Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day2Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day2Part1Alt2Generator;
    impl GeneratorDefault for Day2Part1Alt2Generator {}
    pub struct Day2Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day2Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day2Part1Alt3Generator;
    impl GeneratorDefault for Day2Part1Alt3Generator {}
    pub struct Day2Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day2Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day2Part1Alt4Generator;
    impl GeneratorDefault for Day2Part1Alt4Generator {}
    pub struct Day2Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day2Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day2Part2Alt0Generator;
    impl GeneratorDefault for Day2Part2Alt0Generator {}
    pub struct Day2Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day2Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day2Part2Alt1Generator;
    impl GeneratorDefault for Day2Part2Alt1Generator {}
    pub struct Day2Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day2Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day2Part2Alt2Generator;
    impl GeneratorDefault for Day2Part2Alt2Generator {}
    pub struct Day2Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day2Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day2Part2Alt3Generator;
    impl GeneratorDefault for Day2Part2Alt3Generator {}
    pub struct Day2Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day2Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day2Part2Alt4Generator;
    impl GeneratorDefault for Day2Part2Alt4Generator {}
    pub struct Day2Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day2Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day3Part1Alt0Generator;
    impl GeneratorDefault for Day3Part1Alt0Generator {}
    pub struct Day3Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day3Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day3Part1Alt1Generator;
    impl GeneratorDefault for Day3Part1Alt1Generator {}
    pub struct Day3Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day3Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day3Part1Alt2Generator;
    impl GeneratorDefault for Day3Part1Alt2Generator {}
    pub struct Day3Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day3Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day3Part1Alt3Generator;
    impl GeneratorDefault for Day3Part1Alt3Generator {}
    pub struct Day3Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day3Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day3Part1Alt4Generator;
    impl GeneratorDefault for Day3Part1Alt4Generator {}
    pub struct Day3Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day3Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day3Part2Alt0Generator;
    impl GeneratorDefault for Day3Part2Alt0Generator {}
    pub struct Day3Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day3Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day3Part2Alt1Generator;
    impl GeneratorDefault for Day3Part2Alt1Generator {}
    pub struct Day3Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day3Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day3Part2Alt2Generator;
    impl GeneratorDefault for Day3Part2Alt2Generator {}
    pub struct Day3Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day3Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day3Part2Alt3Generator;
    impl GeneratorDefault for Day3Part2Alt3Generator {}
    pub struct Day3Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day3Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day3Part2Alt4Generator;
    impl GeneratorDefault for Day3Part2Alt4Generator {}
    pub struct Day3Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day3Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day4Part1Alt0Generator;
    impl GeneratorDefault for Day4Part1Alt0Generator {}
    pub struct Day4Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day4Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day4Part1Alt1Generator;
    impl GeneratorDefault for Day4Part1Alt1Generator {}
    pub struct Day4Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day4Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day4Part1Alt2Generator;
    impl GeneratorDefault for Day4Part1Alt2Generator {}
    pub struct Day4Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day4Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day4Part1Alt3Generator;
    impl GeneratorDefault for Day4Part1Alt3Generator {}
    pub struct Day4Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day4Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day4Part1Alt4Generator;
    impl GeneratorDefault for Day4Part1Alt4Generator {}
    pub struct Day4Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day4Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day4Part2Alt0Generator;
    impl GeneratorDefault for Day4Part2Alt0Generator {}
    pub struct Day4Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day4Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day4Part2Alt1Generator;
    impl GeneratorDefault for Day4Part2Alt1Generator {}
    pub struct Day4Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day4Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day4Part2Alt2Generator;
    impl GeneratorDefault for Day4Part2Alt2Generator {}
    pub struct Day4Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day4Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day4Part2Alt3Generator;
    impl GeneratorDefault for Day4Part2Alt3Generator {}
    pub struct Day4Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day4Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day4Part2Alt4Generator;
    impl GeneratorDefault for Day4Part2Alt4Generator {}
    pub struct Day4Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day4Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day5Part1Alt0Generator;
    impl GeneratorDefault for Day5Part1Alt0Generator {}
    pub struct Day5Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day5Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day5Part1Alt1Generator;
    impl GeneratorDefault for Day5Part1Alt1Generator {}
    pub struct Day5Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day5Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day5Part1Alt2Generator;
    impl GeneratorDefault for Day5Part1Alt2Generator {}
    pub struct Day5Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day5Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day5Part1Alt3Generator;
    impl GeneratorDefault for Day5Part1Alt3Generator {}
    pub struct Day5Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day5Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day5Part1Alt4Generator;
    impl GeneratorDefault for Day5Part1Alt4Generator {}
    pub struct Day5Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day5Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day5Part2Alt0Generator;
    impl GeneratorDefault for Day5Part2Alt0Generator {}
    pub struct Day5Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day5Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day5Part2Alt1Generator;
    impl GeneratorDefault for Day5Part2Alt1Generator {}
    pub struct Day5Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day5Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day5Part2Alt2Generator;
    impl GeneratorDefault for Day5Part2Alt2Generator {}
    pub struct Day5Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day5Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day5Part2Alt3Generator;
    impl GeneratorDefault for Day5Part2Alt3Generator {}
    pub struct Day5Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day5Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day5Part2Alt4Generator;
    impl GeneratorDefault for Day5Part2Alt4Generator {}
    pub struct Day5Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day5Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day6Part1Alt0Generator;
    impl GeneratorDefault for Day6Part1Alt0Generator {}
    pub struct Day6Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day6Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day6Part1Alt1Generator;
    impl GeneratorDefault for Day6Part1Alt1Generator {}
    pub struct Day6Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day6Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day6Part1Alt2Generator;
    impl GeneratorDefault for Day6Part1Alt2Generator {}
    pub struct Day6Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day6Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day6Part1Alt3Generator;
    impl GeneratorDefault for Day6Part1Alt3Generator {}
    pub struct Day6Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day6Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day6Part1Alt4Generator;
    impl GeneratorDefault for Day6Part1Alt4Generator {}
    pub struct Day6Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day6Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day6Part2Alt0Generator;
    impl GeneratorDefault for Day6Part2Alt0Generator {}
    pub struct Day6Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day6Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day6Part2Alt1Generator;
    impl GeneratorDefault for Day6Part2Alt1Generator {}
    pub struct Day6Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day6Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day6Part2Alt2Generator;
    impl GeneratorDefault for Day6Part2Alt2Generator {}
    pub struct Day6Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day6Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day6Part2Alt3Generator;
    impl GeneratorDefault for Day6Part2Alt3Generator {}
    pub struct Day6Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day6Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day6Part2Alt4Generator;
    impl GeneratorDefault for Day6Part2Alt4Generator {}
    pub struct Day6Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day6Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day7Part1Alt0Generator;
    impl GeneratorDefault for Day7Part1Alt0Generator {}
    pub struct Day7Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day7Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day7Part1Alt1Generator;
    impl GeneratorDefault for Day7Part1Alt1Generator {}
    pub struct Day7Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day7Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day7Part1Alt2Generator;
    impl GeneratorDefault for Day7Part1Alt2Generator {}
    pub struct Day7Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day7Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day7Part1Alt3Generator;
    impl GeneratorDefault for Day7Part1Alt3Generator {}
    pub struct Day7Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day7Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day7Part1Alt4Generator;
    impl GeneratorDefault for Day7Part1Alt4Generator {}
    pub struct Day7Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day7Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day7Part2Alt0Generator;
    impl GeneratorDefault for Day7Part2Alt0Generator {}
    pub struct Day7Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day7Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day7Part2Alt1Generator;
    impl GeneratorDefault for Day7Part2Alt1Generator {}
    pub struct Day7Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day7Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day7Part2Alt2Generator;
    impl GeneratorDefault for Day7Part2Alt2Generator {}
    pub struct Day7Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day7Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day7Part2Alt3Generator;
    impl GeneratorDefault for Day7Part2Alt3Generator {}
    pub struct Day7Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day7Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day7Part2Alt4Generator;
    impl GeneratorDefault for Day7Part2Alt4Generator {}
    pub struct Day7Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day7Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day8Part1Alt0Generator;
    impl GeneratorDefault for Day8Part1Alt0Generator {}
    pub struct Day8Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day8Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day8Part1Alt1Generator;
    impl GeneratorDefault for Day8Part1Alt1Generator {}
    pub struct Day8Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day8Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day8Part1Alt2Generator;
    impl GeneratorDefault for Day8Part1Alt2Generator {}
    pub struct Day8Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day8Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day8Part1Alt3Generator;
    impl GeneratorDefault for Day8Part1Alt3Generator {}
    pub struct Day8Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day8Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day8Part1Alt4Generator;
    impl GeneratorDefault for Day8Part1Alt4Generator {}
    pub struct Day8Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day8Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day8Part2Alt0Generator;
    impl GeneratorDefault for Day8Part2Alt0Generator {}
    pub struct Day8Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day8Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day8Part2Alt1Generator;
    impl GeneratorDefault for Day8Part2Alt1Generator {}
    pub struct Day8Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day8Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day8Part2Alt2Generator;
    impl GeneratorDefault for Day8Part2Alt2Generator {}
    pub struct Day8Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day8Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day8Part2Alt3Generator;
    impl GeneratorDefault for Day8Part2Alt3Generator {}
    pub struct Day8Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day8Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day8Part2Alt4Generator;
    impl GeneratorDefault for Day8Part2Alt4Generator {}
    pub struct Day8Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day8Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day9Part1Alt0Generator;
    impl GeneratorDefault for Day9Part1Alt0Generator {}
    pub struct Day9Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day9Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day9Part1Alt1Generator;
    impl GeneratorDefault for Day9Part1Alt1Generator {}
    pub struct Day9Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day9Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day9Part1Alt2Generator;
    impl GeneratorDefault for Day9Part1Alt2Generator {}
    pub struct Day9Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day9Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day9Part1Alt3Generator;
    impl GeneratorDefault for Day9Part1Alt3Generator {}
    pub struct Day9Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day9Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day9Part1Alt4Generator;
    impl GeneratorDefault for Day9Part1Alt4Generator {}
    pub struct Day9Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day9Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day9Part2Alt0Generator;
    impl GeneratorDefault for Day9Part2Alt0Generator {}
    pub struct Day9Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day9Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day9Part2Alt1Generator;
    impl GeneratorDefault for Day9Part2Alt1Generator {}
    pub struct Day9Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day9Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day9Part2Alt2Generator;
    impl GeneratorDefault for Day9Part2Alt2Generator {}
    pub struct Day9Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day9Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day9Part2Alt3Generator;
    impl GeneratorDefault for Day9Part2Alt3Generator {}
    pub struct Day9Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day9Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day9Part2Alt4Generator;
    impl GeneratorDefault for Day9Part2Alt4Generator {}
    pub struct Day9Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day9Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day10Part1Alt0Generator;
    impl GeneratorDefault for Day10Part1Alt0Generator {}
    pub struct Day10Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day10Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day10Part1Alt1Generator;
    impl GeneratorDefault for Day10Part1Alt1Generator {}
    pub struct Day10Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day10Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day10Part1Alt2Generator;
    impl GeneratorDefault for Day10Part1Alt2Generator {}
    pub struct Day10Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day10Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day10Part1Alt3Generator;
    impl GeneratorDefault for Day10Part1Alt3Generator {}
    pub struct Day10Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day10Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day10Part1Alt4Generator;
    impl GeneratorDefault for Day10Part1Alt4Generator {}
    pub struct Day10Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day10Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day10Part2Alt0Generator;
    impl GeneratorDefault for Day10Part2Alt0Generator {}
    pub struct Day10Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day10Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day10Part2Alt1Generator;
    impl GeneratorDefault for Day10Part2Alt1Generator {}
    pub struct Day10Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day10Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day10Part2Alt2Generator;
    impl GeneratorDefault for Day10Part2Alt2Generator {}
    pub struct Day10Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day10Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day10Part2Alt3Generator;
    impl GeneratorDefault for Day10Part2Alt3Generator {}
    pub struct Day10Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day10Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day10Part2Alt4Generator;
    impl GeneratorDefault for Day10Part2Alt4Generator {}
    pub struct Day10Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day10Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day11Part1Alt0Generator;
    impl GeneratorDefault for Day11Part1Alt0Generator {}
    pub struct Day11Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day11Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day11Part1Alt1Generator;
    impl GeneratorDefault for Day11Part1Alt1Generator {}
    pub struct Day11Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day11Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day11Part1Alt2Generator;
    impl GeneratorDefault for Day11Part1Alt2Generator {}
    pub struct Day11Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day11Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day11Part1Alt3Generator;
    impl GeneratorDefault for Day11Part1Alt3Generator {}
    pub struct Day11Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day11Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day11Part1Alt4Generator;
    impl GeneratorDefault for Day11Part1Alt4Generator {}
    pub struct Day11Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day11Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day11Part2Alt0Generator;
    impl GeneratorDefault for Day11Part2Alt0Generator {}
    pub struct Day11Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day11Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day11Part2Alt1Generator;
    impl GeneratorDefault for Day11Part2Alt1Generator {}
    pub struct Day11Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day11Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day11Part2Alt2Generator;
    impl GeneratorDefault for Day11Part2Alt2Generator {}
    pub struct Day11Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day11Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day11Part2Alt3Generator;
    impl GeneratorDefault for Day11Part2Alt3Generator {}
    pub struct Day11Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day11Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day11Part2Alt4Generator;
    impl GeneratorDefault for Day11Part2Alt4Generator {}
    pub struct Day11Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day11Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day12Part1Alt0Generator;
    impl GeneratorDefault for Day12Part1Alt0Generator {}
    pub struct Day12Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day12Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day12Part1Alt1Generator;
    impl GeneratorDefault for Day12Part1Alt1Generator {}
    pub struct Day12Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day12Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day12Part1Alt2Generator;
    impl GeneratorDefault for Day12Part1Alt2Generator {}
    pub struct Day12Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day12Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day12Part1Alt3Generator;
    impl GeneratorDefault for Day12Part1Alt3Generator {}
    pub struct Day12Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day12Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day12Part1Alt4Generator;
    impl GeneratorDefault for Day12Part1Alt4Generator {}
    pub struct Day12Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day12Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day12Part2Alt0Generator;
    impl GeneratorDefault for Day12Part2Alt0Generator {}
    pub struct Day12Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day12Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day12Part2Alt1Generator;
    impl GeneratorDefault for Day12Part2Alt1Generator {}
    pub struct Day12Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day12Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day12Part2Alt2Generator;
    impl GeneratorDefault for Day12Part2Alt2Generator {}
    pub struct Day12Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day12Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day12Part2Alt3Generator;
    impl GeneratorDefault for Day12Part2Alt3Generator {}
    pub struct Day12Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day12Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day12Part2Alt4Generator;
    impl GeneratorDefault for Day12Part2Alt4Generator {}
    pub struct Day12Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day12Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day13Part1Alt0Generator;
    impl GeneratorDefault for Day13Part1Alt0Generator {}
    pub struct Day13Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day13Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day13Part1Alt1Generator;
    impl GeneratorDefault for Day13Part1Alt1Generator {}
    pub struct Day13Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day13Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day13Part1Alt2Generator;
    impl GeneratorDefault for Day13Part1Alt2Generator {}
    pub struct Day13Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day13Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day13Part1Alt3Generator;
    impl GeneratorDefault for Day13Part1Alt3Generator {}
    pub struct Day13Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day13Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day13Part1Alt4Generator;
    impl GeneratorDefault for Day13Part1Alt4Generator {}
    pub struct Day13Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day13Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day13Part2Alt0Generator;
    impl GeneratorDefault for Day13Part2Alt0Generator {}
    pub struct Day13Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day13Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day13Part2Alt1Generator;
    impl GeneratorDefault for Day13Part2Alt1Generator {}
    pub struct Day13Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day13Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day13Part2Alt2Generator;
    impl GeneratorDefault for Day13Part2Alt2Generator {}
    pub struct Day13Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day13Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day13Part2Alt3Generator;
    impl GeneratorDefault for Day13Part2Alt3Generator {}
    pub struct Day13Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day13Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day13Part2Alt4Generator;
    impl GeneratorDefault for Day13Part2Alt4Generator {}
    pub struct Day13Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day13Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day14Part1Alt0Generator;
    impl GeneratorDefault for Day14Part1Alt0Generator {}
    pub struct Day14Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day14Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day14Part1Alt1Generator;
    impl GeneratorDefault for Day14Part1Alt1Generator {}
    pub struct Day14Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day14Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day14Part1Alt2Generator;
    impl GeneratorDefault for Day14Part1Alt2Generator {}
    pub struct Day14Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day14Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day14Part1Alt3Generator;
    impl GeneratorDefault for Day14Part1Alt3Generator {}
    pub struct Day14Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day14Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day14Part1Alt4Generator;
    impl GeneratorDefault for Day14Part1Alt4Generator {}
    pub struct Day14Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day14Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day14Part2Alt0Generator;
    impl GeneratorDefault for Day14Part2Alt0Generator {}
    pub struct Day14Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day14Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day14Part2Alt1Generator;
    impl GeneratorDefault for Day14Part2Alt1Generator {}
    pub struct Day14Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day14Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day14Part2Alt2Generator;
    impl GeneratorDefault for Day14Part2Alt2Generator {}
    pub struct Day14Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day14Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day14Part2Alt3Generator;
    impl GeneratorDefault for Day14Part2Alt3Generator {}
    pub struct Day14Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day14Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day14Part2Alt4Generator;
    impl GeneratorDefault for Day14Part2Alt4Generator {}
    pub struct Day14Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day14Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day15Part1Alt0Generator;
    impl GeneratorDefault for Day15Part1Alt0Generator {}
    pub struct Day15Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day15Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day15Part1Alt1Generator;
    impl GeneratorDefault for Day15Part1Alt1Generator {}
    pub struct Day15Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day15Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day15Part1Alt2Generator;
    impl GeneratorDefault for Day15Part1Alt2Generator {}
    pub struct Day15Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day15Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day15Part1Alt3Generator;
    impl GeneratorDefault for Day15Part1Alt3Generator {}
    pub struct Day15Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day15Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day15Part1Alt4Generator;
    impl GeneratorDefault for Day15Part1Alt4Generator {}
    pub struct Day15Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day15Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day15Part2Alt0Generator;
    impl GeneratorDefault for Day15Part2Alt0Generator {}
    pub struct Day15Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day15Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day15Part2Alt1Generator;
    impl GeneratorDefault for Day15Part2Alt1Generator {}
    pub struct Day15Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day15Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day15Part2Alt2Generator;
    impl GeneratorDefault for Day15Part2Alt2Generator {}
    pub struct Day15Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day15Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day15Part2Alt3Generator;
    impl GeneratorDefault for Day15Part2Alt3Generator {}
    pub struct Day15Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day15Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day15Part2Alt4Generator;
    impl GeneratorDefault for Day15Part2Alt4Generator {}
    pub struct Day15Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day15Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day16Part1Alt0Generator;
    impl GeneratorDefault for Day16Part1Alt0Generator {}
    pub struct Day16Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day16Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day16Part1Alt1Generator;
    impl GeneratorDefault for Day16Part1Alt1Generator {}
    pub struct Day16Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day16Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day16Part1Alt2Generator;
    impl GeneratorDefault for Day16Part1Alt2Generator {}
    pub struct Day16Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day16Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day16Part1Alt3Generator;
    impl GeneratorDefault for Day16Part1Alt3Generator {}
    pub struct Day16Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day16Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day16Part1Alt4Generator;
    impl GeneratorDefault for Day16Part1Alt4Generator {}
    pub struct Day16Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day16Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day16Part2Alt0Generator;
    impl GeneratorDefault for Day16Part2Alt0Generator {}
    pub struct Day16Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day16Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day16Part2Alt1Generator;
    impl GeneratorDefault for Day16Part2Alt1Generator {}
    pub struct Day16Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day16Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day16Part2Alt2Generator;
    impl GeneratorDefault for Day16Part2Alt2Generator {}
    pub struct Day16Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day16Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day16Part2Alt3Generator;
    impl GeneratorDefault for Day16Part2Alt3Generator {}
    pub struct Day16Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day16Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day16Part2Alt4Generator;
    impl GeneratorDefault for Day16Part2Alt4Generator {}
    pub struct Day16Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day16Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day17Part1Alt0Generator;
    impl GeneratorDefault for Day17Part1Alt0Generator {}
    pub struct Day17Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day17Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day17Part1Alt1Generator;
    impl GeneratorDefault for Day17Part1Alt1Generator {}
    pub struct Day17Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day17Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day17Part1Alt2Generator;
    impl GeneratorDefault for Day17Part1Alt2Generator {}
    pub struct Day17Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day17Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day17Part1Alt3Generator;
    impl GeneratorDefault for Day17Part1Alt3Generator {}
    pub struct Day17Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day17Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day17Part1Alt4Generator;
    impl GeneratorDefault for Day17Part1Alt4Generator {}
    pub struct Day17Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day17Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day17Part2Alt0Generator;
    impl GeneratorDefault for Day17Part2Alt0Generator {}
    pub struct Day17Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day17Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day17Part2Alt1Generator;
    impl GeneratorDefault for Day17Part2Alt1Generator {}
    pub struct Day17Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day17Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day17Part2Alt2Generator;
    impl GeneratorDefault for Day17Part2Alt2Generator {}
    pub struct Day17Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day17Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day17Part2Alt3Generator;
    impl GeneratorDefault for Day17Part2Alt3Generator {}
    pub struct Day17Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day17Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day17Part2Alt4Generator;
    impl GeneratorDefault for Day17Part2Alt4Generator {}
    pub struct Day17Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day17Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day18Part1Alt0Generator;
    impl GeneratorDefault for Day18Part1Alt0Generator {}
    pub struct Day18Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day18Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day18Part1Alt1Generator;
    impl GeneratorDefault for Day18Part1Alt1Generator {}
    pub struct Day18Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day18Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day18Part1Alt2Generator;
    impl GeneratorDefault for Day18Part1Alt2Generator {}
    pub struct Day18Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day18Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day18Part1Alt3Generator;
    impl GeneratorDefault for Day18Part1Alt3Generator {}
    pub struct Day18Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day18Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day18Part1Alt4Generator;
    impl GeneratorDefault for Day18Part1Alt4Generator {}
    pub struct Day18Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day18Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day18Part2Alt0Generator;
    impl GeneratorDefault for Day18Part2Alt0Generator {}
    pub struct Day18Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day18Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day18Part2Alt1Generator;
    impl GeneratorDefault for Day18Part2Alt1Generator {}
    pub struct Day18Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day18Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day18Part2Alt2Generator;
    impl GeneratorDefault for Day18Part2Alt2Generator {}
    pub struct Day18Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day18Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day18Part2Alt3Generator;
    impl GeneratorDefault for Day18Part2Alt3Generator {}
    pub struct Day18Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day18Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day18Part2Alt4Generator;
    impl GeneratorDefault for Day18Part2Alt4Generator {}
    pub struct Day18Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day18Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day19Part1Alt0Generator;
    impl GeneratorDefault for Day19Part1Alt0Generator {}
    pub struct Day19Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day19Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day19Part1Alt1Generator;
    impl GeneratorDefault for Day19Part1Alt1Generator {}
    pub struct Day19Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day19Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day19Part1Alt2Generator;
    impl GeneratorDefault for Day19Part1Alt2Generator {}
    pub struct Day19Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day19Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day19Part1Alt3Generator;
    impl GeneratorDefault for Day19Part1Alt3Generator {}
    pub struct Day19Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day19Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day19Part1Alt4Generator;
    impl GeneratorDefault for Day19Part1Alt4Generator {}
    pub struct Day19Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day19Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day19Part2Alt0Generator;
    impl GeneratorDefault for Day19Part2Alt0Generator {}
    pub struct Day19Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day19Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day19Part2Alt1Generator;
    impl GeneratorDefault for Day19Part2Alt1Generator {}
    pub struct Day19Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day19Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day19Part2Alt2Generator;
    impl GeneratorDefault for Day19Part2Alt2Generator {}
    pub struct Day19Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day19Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day19Part2Alt3Generator;
    impl GeneratorDefault for Day19Part2Alt3Generator {}
    pub struct Day19Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day19Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day19Part2Alt4Generator;
    impl GeneratorDefault for Day19Part2Alt4Generator {}
    pub struct Day19Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day19Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day20Part1Alt0Generator;
    impl GeneratorDefault for Day20Part1Alt0Generator {}
    pub struct Day20Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day20Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day20Part1Alt1Generator;
    impl GeneratorDefault for Day20Part1Alt1Generator {}
    pub struct Day20Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day20Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day20Part1Alt2Generator;
    impl GeneratorDefault for Day20Part1Alt2Generator {}
    pub struct Day20Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day20Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day20Part1Alt3Generator;
    impl GeneratorDefault for Day20Part1Alt3Generator {}
    pub struct Day20Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day20Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day20Part1Alt4Generator;
    impl GeneratorDefault for Day20Part1Alt4Generator {}
    pub struct Day20Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day20Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day20Part2Alt0Generator;
    impl GeneratorDefault for Day20Part2Alt0Generator {}
    pub struct Day20Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day20Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day20Part2Alt1Generator;
    impl GeneratorDefault for Day20Part2Alt1Generator {}
    pub struct Day20Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day20Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day20Part2Alt2Generator;
    impl GeneratorDefault for Day20Part2Alt2Generator {}
    pub struct Day20Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day20Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day20Part2Alt3Generator;
    impl GeneratorDefault for Day20Part2Alt3Generator {}
    pub struct Day20Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day20Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day20Part2Alt4Generator;
    impl GeneratorDefault for Day20Part2Alt4Generator {}
    pub struct Day20Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day20Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day21Part1Alt0Generator;
    impl GeneratorDefault for Day21Part1Alt0Generator {}
    pub struct Day21Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day21Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day21Part1Alt1Generator;
    impl GeneratorDefault for Day21Part1Alt1Generator {}
    pub struct Day21Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day21Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day21Part1Alt2Generator;
    impl GeneratorDefault for Day21Part1Alt2Generator {}
    pub struct Day21Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day21Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day21Part1Alt3Generator;
    impl GeneratorDefault for Day21Part1Alt3Generator {}
    pub struct Day21Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day21Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day21Part1Alt4Generator;
    impl GeneratorDefault for Day21Part1Alt4Generator {}
    pub struct Day21Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day21Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day21Part2Alt0Generator;
    impl GeneratorDefault for Day21Part2Alt0Generator {}
    pub struct Day21Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day21Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day21Part2Alt1Generator;
    impl GeneratorDefault for Day21Part2Alt1Generator {}
    pub struct Day21Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day21Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day21Part2Alt2Generator;
    impl GeneratorDefault for Day21Part2Alt2Generator {}
    pub struct Day21Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day21Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day21Part2Alt3Generator;
    impl GeneratorDefault for Day21Part2Alt3Generator {}
    pub struct Day21Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day21Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day21Part2Alt4Generator;
    impl GeneratorDefault for Day21Part2Alt4Generator {}
    pub struct Day21Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day21Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day22Part1Alt0Generator;
    impl GeneratorDefault for Day22Part1Alt0Generator {}
    pub struct Day22Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day22Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day22Part1Alt1Generator;
    impl GeneratorDefault for Day22Part1Alt1Generator {}
    pub struct Day22Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day22Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day22Part1Alt2Generator;
    impl GeneratorDefault for Day22Part1Alt2Generator {}
    pub struct Day22Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day22Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day22Part1Alt3Generator;
    impl GeneratorDefault for Day22Part1Alt3Generator {}
    pub struct Day22Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day22Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day22Part1Alt4Generator;
    impl GeneratorDefault for Day22Part1Alt4Generator {}
    pub struct Day22Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day22Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day22Part2Alt0Generator;
    impl GeneratorDefault for Day22Part2Alt0Generator {}
    pub struct Day22Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day22Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day22Part2Alt1Generator;
    impl GeneratorDefault for Day22Part2Alt1Generator {}
    pub struct Day22Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day22Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day22Part2Alt2Generator;
    impl GeneratorDefault for Day22Part2Alt2Generator {}
    pub struct Day22Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day22Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day22Part2Alt3Generator;
    impl GeneratorDefault for Day22Part2Alt3Generator {}
    pub struct Day22Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day22Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day22Part2Alt4Generator;
    impl GeneratorDefault for Day22Part2Alt4Generator {}
    pub struct Day22Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day22Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day23Part1Alt0Generator;
    impl GeneratorDefault for Day23Part1Alt0Generator {}
    pub struct Day23Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day23Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day23Part1Alt1Generator;
    impl GeneratorDefault for Day23Part1Alt1Generator {}
    pub struct Day23Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day23Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day23Part1Alt2Generator;
    impl GeneratorDefault for Day23Part1Alt2Generator {}
    pub struct Day23Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day23Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day23Part1Alt3Generator;
    impl GeneratorDefault for Day23Part1Alt3Generator {}
    pub struct Day23Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day23Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day23Part1Alt4Generator;
    impl GeneratorDefault for Day23Part1Alt4Generator {}
    pub struct Day23Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day23Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day23Part2Alt0Generator;
    impl GeneratorDefault for Day23Part2Alt0Generator {}
    pub struct Day23Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day23Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day23Part2Alt1Generator;
    impl GeneratorDefault for Day23Part2Alt1Generator {}
    pub struct Day23Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day23Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day23Part2Alt2Generator;
    impl GeneratorDefault for Day23Part2Alt2Generator {}
    pub struct Day23Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day23Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day23Part2Alt3Generator;
    impl GeneratorDefault for Day23Part2Alt3Generator {}
    pub struct Day23Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day23Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day23Part2Alt4Generator;
    impl GeneratorDefault for Day23Part2Alt4Generator {}
    pub struct Day23Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day23Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day24Part1Alt0Generator;
    impl GeneratorDefault for Day24Part1Alt0Generator {}
    pub struct Day24Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day24Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day24Part1Alt1Generator;
    impl GeneratorDefault for Day24Part1Alt1Generator {}
    pub struct Day24Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day24Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day24Part1Alt2Generator;
    impl GeneratorDefault for Day24Part1Alt2Generator {}
    pub struct Day24Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day24Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day24Part1Alt3Generator;
    impl GeneratorDefault for Day24Part1Alt3Generator {}
    pub struct Day24Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day24Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day24Part1Alt4Generator;
    impl GeneratorDefault for Day24Part1Alt4Generator {}
    pub struct Day24Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day24Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day24Part2Alt0Generator;
    impl GeneratorDefault for Day24Part2Alt0Generator {}
    pub struct Day24Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day24Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day24Part2Alt1Generator;
    impl GeneratorDefault for Day24Part2Alt1Generator {}
    pub struct Day24Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day24Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day24Part2Alt2Generator;
    impl GeneratorDefault for Day24Part2Alt2Generator {}
    pub struct Day24Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day24Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day24Part2Alt3Generator;
    impl GeneratorDefault for Day24Part2Alt3Generator {}
    pub struct Day24Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day24Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day24Part2Alt4Generator;
    impl GeneratorDefault for Day24Part2Alt4Generator {}
    pub struct Day24Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day24Part2Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day25Part1Alt0Generator;
    impl GeneratorDefault for Day25Part1Alt0Generator {}
    pub struct Day25Part1Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day25Part1Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day25Part1Alt1Generator;
    impl GeneratorDefault for Day25Part1Alt1Generator {}
    pub struct Day25Part1Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day25Part1Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day25Part1Alt2Generator;
    impl GeneratorDefault for Day25Part1Alt2Generator {}
    pub struct Day25Part1Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day25Part1Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day25Part1Alt3Generator;
    impl GeneratorDefault for Day25Part1Alt3Generator {}
    pub struct Day25Part1Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day25Part1Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day25Part1Alt4Generator;
    impl GeneratorDefault for Day25Part1Alt4Generator {}
    pub struct Day25Part1Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day25Part1Alt4Runner<I> {
        type Input = I;
    }
    pub struct Day25Part2Alt0Generator;
    impl GeneratorDefault for Day25Part2Alt0Generator {}
    pub struct Day25Part2Alt0Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day25Part2Alt0Runner<I> {
        type Input = I;
    }
    pub struct Day25Part2Alt1Generator;
    impl GeneratorDefault for Day25Part2Alt1Generator {}
    pub struct Day25Part2Alt1Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day25Part2Alt1Runner<I> {
        type Input = I;
    }
    pub struct Day25Part2Alt2Generator;
    impl GeneratorDefault for Day25Part2Alt2Generator {}
    pub struct Day25Part2Alt2Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day25Part2Alt2Runner<I> {
        type Input = I;
    }
    pub struct Day25Part2Alt3Generator;
    impl GeneratorDefault for Day25Part2Alt3Generator {}
    pub struct Day25Part2Alt3Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day25Part2Alt3Runner<I> {
        type Input = I;
    }
    pub struct Day25Part2Alt4Generator;
    impl GeneratorDefault for Day25Part2Alt4Generator {}
    pub struct Day25Part2Alt4Runner<I>(pub PhantomData<I>);
    impl<I> RunnerDefault for Day25Part2Alt4Runner<I> {
        type Input = I;
    }
}
