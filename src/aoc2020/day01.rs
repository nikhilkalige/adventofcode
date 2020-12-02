use itertools::Itertools;

pub fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .parse::<u32>()
                .expect("Input should only contain unsinged integers")
        })
        .collect()
}

pub fn part1(numbers: &[u32]) -> u64 {
    product_of_combinations(numbers, 2)
}

pub fn part2(numbers: &[u32]) -> u64 {
    product_of_combinations(numbers, 3)
}

fn product_of_combinations(numbers: &[u32], num_elements: usize) -> u64 {
    let mut sum = numbers
        .iter()
        .combinations(num_elements)
        .map(|elements| {
            let product = elements.iter().map(|e| **e as u64).product::<u64>();
            let sum = elements.iter().map(|e| **e).sum::<u32>();
            (sum, product)
        })
        .filter(|&(sum, _product)| sum == 2020);
    let result = sum.next().expect("Expected atleast one valid answer");
    result.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_of_combinations() {
        let numbers: Vec<_> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(product_of_combinations(&numbers, 2), 514579);
        assert_eq!(product_of_combinations(&numbers, 3), 241861950);
    }
}
