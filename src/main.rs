use aoc_derive::aoc_run;
use std::io::{self};

mod aoc2020;

fn main() -> io::Result<()> {
    // let input = include_str!(concat!(
    //     env!("CARGO_MANIFEST_DIR"),
    //     "/src/aoc2020/input/day01"
    // ));

    // let input = aoc2020::day01::parse(input);
    // println!("Sum: {}", aoc2020::day01::part1(&input));
    // println!("Sum: {}", aoc2020::day01::part2(&input));

    // let input = include_str!(concat!(
    //     env!("CARGO_MANIFEST_DIR"),
    //     "/src/aoc2020/input/day02"
    // ));

    // let input = aoc2020::day02::parse(input);
    // println!("Sum: {}", aoc2020::day02::part1(&input));
    // println!("Sum: {}", aoc2020::day02::part2(&input));

    aoc_run!(2020, 1 , 2);

    Ok(())
}
