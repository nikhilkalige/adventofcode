// use aoc_derive::{aoc_parser, aoc_solver};
// use std::{collections::HashMap, str::FromStr};

// #[aoc_parser(day02)]
// pub fn parse(input: &str) -> Result<Vec<(PasswordReq, String)>, String> {
//     Ok(input.lines().map(|s| parse_line(s)).collect())
// }

// #[aoc_solver(day02, part1)]
// pub fn part1(input: &[(PasswordReq, String)]) -> usize {
//     input
//         .iter()
//         .filter(|(req, password)| password_count_test(req, password))
//         .count()
// }

// #[aoc_solver(day02, part2)]
// pub fn part2(input: &[(PasswordReq, String)]) -> usize {
//     input
//         .iter()
//         .filter(|(req, password)| password_position_test(req, password))
//         .count()
// }

// fn password_count_test(req: &PasswordReq, password: &str) -> bool {
//     let mut characters = HashMap::new();
//     for c in password.chars() {
//         *characters.entry(c).or_insert(0) += 1;
//     }

//     match characters.get(&req.character) {
//         Some(count) => {
//             return (req.repeats.0 <= *count) && (*count <= req.repeats.1);
//         }
//         None => false,
//     }
// }

// fn password_position_test(req: &PasswordReq, password: &str) -> bool {
//     let has_character = |iterator: &str, pos| {
//         let value = iterator.chars().nth((pos - 1) as usize);
//         match value {
//             Some(c) => c == req.character,
//             None => false,
//         }
//     };

//     let first = has_character(password, req.repeats.0);
//     let second = has_character(password, req.repeats.1);

//     first ^ second
// }

// fn parse_line(line: &str) -> (PasswordReq, String) {
//     let mut it = line.split(':');
//     let first = it.next().expect("Unexpected format");
//     let rest = it.next().expect("Unexpected format").trim();

//     let req = first.parse::<PasswordReq>().unwrap();
//     (req, rest.to_owned())
// }

// #[derive(Debug, Clone, PartialEq)]
// pub struct PasswordReq {
//     repeats: (u32, u32),
//     character: char,
// }

// impl FromStr for PasswordReq {
//     type Err = i32;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut iter = s.split(' ');
//         let repeat_info = iter.next().expect("Unexpected data");
//         let mut numbers = repeat_info
//             .split('-')
//             .map(|c| c.parse::<u32>().expect("Unexpected format"));
//         let min = numbers.next().expect("Unexpected format");
//         let max = numbers.next().expect("Unexpected format");
//         let character = iter
//             .next()
//             .expect("Unexpected data")
//             .chars()
//             .next()
//             .expect("Unexpected format");

//         Ok(PasswordReq {
//             repeats: (min, max),
//             character,
//         })
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_line() {
//         let (req, pass) = parse_line("1-3 a: abcde");
//         assert_eq!(
//             req,
//             PasswordReq {
//                 repeats: (1, 3),
//                 character: 'a'
//             },
//             "abcde"
//         );
//         assert_eq!(pass, "abcde");

//         let (req, pass) = parse_line("12-35 a: ab");
//         assert_eq!(
//             req,
//             PasswordReq {
//                 repeats: (12, 35),
//                 character: 'a'
//             },
//             "ab"
//         );
//         assert_eq!(pass, "ab");
//     }

//     #[test]
//     fn test_by_password_count() {
//         let (req, password) = parse_line("1-3 a: abcde");
//         assert!(password_count_test(&req, &password));
//         let (req, password) = parse_line("1-3 b: cdefg");
//         assert!(!password_count_test(&req, &password));
//         let (req, password) = parse_line("2-9 c: ccccccccc");
//         assert!(password_count_test(&req, &password));
//     }

//     #[test]
//     fn test_by_password_position() {
//         let (req, password) = parse_line("1-3 a: abcde");
//         assert!(password_position_test(&req, &password));
//         let (req, password) = parse_line("1-3 b: cdefg");
//         assert!(!password_position_test(&req, &password));
//         let (req, password) = parse_line("2-9 c: ccccccccc");
//         assert!(!password_position_test(&req, &password));
//     }
// }
