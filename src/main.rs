use std::io::{self};

mod aoc2020;

fn main() -> io::Result<()> {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/aoc2020/input/day01"));

    let input = aoc2020::day01::parse(input);
    println!("Sum: {}", aoc2020::day01::part1(&input));
    println!("Sum: {}", aoc2020::day01::part2(&input));

    // let opcodes: Vec<i64> = input
    //     .trim()
    //     .split(',')
    //     .map(|s| s.parse().expect("Input should be a number"))
    //     .collect();

    // {
    //     let opcodes = opcodes.clone();
    //     let (_, result) = simple_processor(&opcodes, vec![1]);
    //     println!("{:?}", result);
    // }
    // {
    //     let opcodes = opcodes.clone();
    //     let (_, result) = simple_processor(&opcodes, vec![5]);
    //     println!("{:?}", result);
    // }

    Ok(())
}
