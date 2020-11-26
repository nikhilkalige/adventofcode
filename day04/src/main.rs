use itertools::Itertools;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut numbers = input.trim().split('-').map(|s| s.parse::<u32>().unwrap());

    // Part 1
    let results = pattern(numbers.next().unwrap(), numbers.next().unwrap());
    println!("Number of patterns: {}", results.len());

    // Part2
    let count = part2(&results);
    println!("Number of non adjacent: {}", count);

    Ok(())
}

fn part2(numbers: &[u32]) -> usize {
    let mut results = Vec::<u32>::new();
    for number in numbers {
        let digits = split_number(*number);
        if non_adjacent_doubles(&digits) {
            results.push(*number);
        }
    }
    results.len()
}

fn pattern(start: u32, end: u32) -> Vec<u32> {
    let mut results = Vec::<u32>::new();
    for number in start..end + 1 {
        let digits = split_number(number);
        if is_increasing(&digits) && has_doubles(&digits) {
            results.push(number);
        }
    }
    results
}

fn split_number(n: u32) -> Vec<u32> {
    let mut result = Vec::<u32>::new();
    let mut quotient = n;
    while quotient != 0 {
        result.push(quotient % 10);
        quotient = quotient / 10;
    }
    result.reverse();
    result
}

fn is_increasing(digits: &[u32]) -> bool {
    for (a, b) in digits.iter().tuple_combinations() {
        if b < a {
            return false;
        }
    }
    true
}

fn has_doubles(digits: &[u32]) -> bool {
    for (a, b) in digits.iter().tuple_combinations() {
        if a == b {
            return true;
        }
    }
    false
}

fn non_adjacent_doubles(digits: &[u32]) -> bool {
    let mut current_index: usize = 0;
    let mut num_matches = vec![1; digits.len()];
    for (a, b) in digits.iter().tuple_windows() {
        if a == b {
            num_matches[current_index] += 1;
        } else {
            current_index += 1;
        }
    }

    let duals = num_matches.iter().filter(|x| **x == 2);
    let result = duals.collect_vec().len() != 0;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_number() {
        assert_eq!(split_number(1), vec![1]);
        assert_eq!(split_number(21), vec![2, 1]);
        assert_eq!(split_number(9880), vec![9, 8, 8, 0]);
    }

    #[test]
    fn test_is_increasing() {
        assert!(is_increasing(&vec![1, 2]));
        assert!(!is_increasing(&vec![2, 1]));
        assert!(!is_increasing(&vec![1, 3, 2]));
        assert!(is_increasing(&vec![3, 3, 5]));
    }

    #[test]
    fn test_has_doubles() {
        assert!(!has_doubles(&vec![1, 2]));
        assert!(has_doubles(&vec![2, 2]));
        assert!(!has_doubles(&vec![1, 3, 2]));
        assert!(has_doubles(&vec![4, 5, 5, 0]));
    }

    #[test]
    fn test_non_adjacent_doubles() {
        assert!(non_adjacent_doubles(&vec![1, 1, 2, 2, 3, 3]));
        assert!(!non_adjacent_doubles(&vec![1, 2, 3, 4, 4, 4]));
        assert!(non_adjacent_doubles(&vec![1, 1, 1, 1, 2, 2]));
        assert!(non_adjacent_doubles(&vec![1, 6, 6, 7, 7, 7]));
    }
}
