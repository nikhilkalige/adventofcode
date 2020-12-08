use std::str::FromStr;
use std::{
    convert::TryFrom,
    fmt::{Display, Error, Formatter},
};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Day {
    Day01 = 1,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl FromStr for Day {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Day1" | "Day01" | "day1" | "day01" | "D1" | "d1" => Day::Day01,
            "Day2" | "Day02" | "day2" | "day02" | "D2" | "d2" => Day::Day02,
            "Day3" | "Day03" | "day3" | "day03" | "D3" | "d3" => Day::Day03,
            "Day4" | "Day04" | "day4" | "day04" | "D4" | "d4" => Day::Day04,
            "Day5" | "Day05" | "day5" | "day05" | "D5" | "d5" => Day::Day05,
            "Day6" | "Day06" | "day6" | "day06" | "D6" | "d6" => Day::Day06,
            "Day7" | "Day07" | "day7" | "day07" | "D7" | "d7" => Day::Day07,
            "Day8" | "Day08" | "day8" | "day08" | "D8" | "d8" => Day::Day08,
            "Day9" | "Day09" | "day9" | "day09" | "D9" | "d9" => Day::Day09,
            "Day10" | "day10" | "D10" | "d10" => Day::Day10,
            "Day11" | "day11" | "D11" | "d11" => Day::Day11,
            "Day12" | "day12" | "D12" | "d12" => Day::Day12,
            "Day13" | "day13" | "D13" | "d13" => Day::Day13,
            "Day14" | "day14" | "D14" | "d14" => Day::Day14,
            "Day15" | "day15" | "D15" | "d15" => Day::Day15,
            "Day16" | "day16" | "D16" | "d16" => Day::Day16,
            "Day17" | "day17" | "D17" | "d17" => Day::Day17,
            "Day18" | "day18" | "D18" | "d18" => Day::Day18,
            "Day19" | "day19" | "D19" | "d19" => Day::Day19,
            "Day20" | "day20" | "D20" | "d20" => Day::Day20,
            "Day21" | "day21" | "D21" | "d21" => Day::Day21,
            "Day22" | "day22" | "D22" | "d22" => Day::Day22,
            "Day23" | "day23" | "D23" | "d23" => Day::Day23,
            "Day24" | "day24" | "D24" | "d24" => Day::Day24,
            "Day25" | "day25" | "D25" | "d25" => Day::Day25,
            _ => {
                return Err(
                    "Failed to parse day, allowed patterns: DayX, dayX, DX, dX with X in 1..=25",
                )
            }
        })
    }
}

impl TryFrom<u32> for Day {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            01 => Day::Day01,
            02 => Day::Day02,
            03 => Day::Day03,
            04 => Day::Day04,
            05 => Day::Day05,
            06 => Day::Day06,
            07 => Day::Day07,
            08 => Day::Day08,
            09 => Day::Day09,
            10 => Day::Day10,
            11 => Day::Day11,
            12 => Day::Day12,
            13 => Day::Day13,
            14 => Day::Day14,
            15 => Day::Day15,
            16 => Day::Day16,
            17 => Day::Day17,
            18 => Day::Day18,
            19 => Day::Day19,
            20 => Day::Day20,
            21 => Day::Day21,
            22 => Day::Day22,
            23 => Day::Day23,
            24 => Day::Day24,
            25 => Day::Day25,
            _ => {
                return Err("Day must be in range 1..=25");
            }
        })
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Day{:02}", *self as u8)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Part {
    Part1 = 1,
    Part2,
}

impl Part {
    pub fn variants() -> &'static [Part] {
        &[Part::Part1, Part::Part2]
    }
}

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        Ok(match val {
            "Part1" | "part1" | "P1" | "p1" => Part::Part1,
            "Part2" | "part2" | "P2" | "p2" => Part::Part2,
            _ => {
                return Err(
                    "Failed to parse part, allowed patterns: PartX, partX, PX, pX with X in 1..=2",
                )
            }
        })
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Part{}", *self as u8)
    }
}

impl TryFrom<u32> for Part {
    type Error = &'static str;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        Ok(match val {
            1 => Part::Part1,
            2 => Part::Part2,
            _ => return Err("Part must be in range 1..=2"),
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Solution {
    Solution1 = 1,
    Solution2,
    Solution3,
    Solution4,
}

impl Solution {
    pub fn variants() -> &'static [Solution] {
        &[
            Solution::Solution1,
            Solution::Solution2,
            Solution::Solution3,
            Solution::Solution4,
        ]
    }
}

impl FromStr for Solution {
    type Err = &'static str;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        Ok(match val {
            "Solution1" | "solution1" | "S1" | "s1" => Solution::Solution1,
            "Solution2" | "solution2" | "S2" | "s2" => Solution::Solution2,
            "Solution3" | "solution3" | "S3" | "s3" => Solution::Solution3,
            "Solution4" | "solution4" | "S4" | "s4" => Solution::Solution4,
            _ => {
                return Err(
                    "Failed to parse solution, allowed patterns: SolutionX, SolutionX, SX, sX with X in 1..=3",
                )
            }
        })
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Solution{}", *self as u8)
    }
}

impl TryFrom<u32> for Solution {
    type Error = &'static str;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        Ok(match val {
            1 => Solution::Solution1,
            2 => Solution::Solution2,
            3 => Solution::Solution3,
            4 => Solution::Solution4,
            _ => return Err("Solution must be in range 1..=4"),
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct DayPart<'a> {
    pub day: Day,
    pub part: Part,
    pub solution: Solution,
    pub name: Option<&'a str>,
}

#[derive(Debug, Clone, Copy)]
pub enum ParserResultType {
    Option,
    Result,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_conversions() {
        assert_eq!(Ok(Day::Day01), Day::try_from(1));
        assert_eq!(Day::Day01, Day::try_from(1).unwrap());
        assert_eq!(Day::Day02 as u8, 1);
        assert_eq!(Ok(Part::Part1), Part::try_from(1));
    }
}
