use itertools::Itertools;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let opcodes: Vec<u32> = input
        .split(',')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect();

    // println!("{:?}, {}", opcodes, opcodes.len());

    // Part 1
    {
        let mut new_opcodes = opcodes.clone();
        new_opcodes[1] = 12;
        new_opcodes[2] = 2;
        processor(&mut new_opcodes);
        println!("Corrected opcode: {}", new_opcodes[0]);
    }

    // Part2
    {
        const EXPECTED_OPCODE: u32 = 19690720;
        match opcode_finder(&opcodes, EXPECTED_OPCODE) {
            Some((noun, verb)) => {
                println!(
                    "Result: Noun {}, Verb {}, Opcode: {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
            }
            None => {}
        }
    }

    Ok(())
}

fn processor(opcodes: &mut [u32]) {
    for i in (0..opcodes.len()).filter(|n| n % 4 == 0) {
        let op = opcodes[i + 0];

        if op == 1 {
            let (a, b, r) = (
                opcodes[i + 1] as usize,
                opcodes[i + 2] as usize,
                opcodes[i + 3] as usize,
            );
            let result = opcodes[a] + opcodes[b];
            opcodes[r] = result;
        } else if op == 2 {
            let (a, b, r) = (
                opcodes[i + 1] as usize,
                opcodes[i + 2] as usize,
                opcodes[i + 3] as usize,
            );
            let result = opcodes[a] * opcodes[b];
            opcodes[r] = result;
        } else if op == 99 {
            return;
        } else {
            println!("Got invalid opcode {}", op);
            return;
        }
    }
}

fn opcode_finder(opcodes: &[u32], expected: u32) -> Option<(u32, u32)> {
    for perms in (0..100).permutations(2) {
        let mut new_opcodes = opcodes.clone().to_owned();

        if let [noun, verb] = perms[..] {
            new_opcodes[1] = noun;
            new_opcodes[2] = verb;

            processor(&mut new_opcodes);

            let opcode = new_opcodes[0];
            if opcode == expected {
                return Some((noun, verb));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor() {
        {
            let mut opcodes: Vec<u32> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
            processor(&mut opcodes);
            assert_eq!(vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50], opcodes);
        }
        {
            let mut opcodes: Vec<u32> = vec![1, 0, 0, 0, 99];
            processor(&mut opcodes);
            assert_eq!(vec![2, 0, 0, 0, 99], opcodes);
        }
        {
            let mut opcodes: Vec<u32> = vec![2, 3, 0, 3, 99];
            processor(&mut opcodes);
            assert_eq!(vec![2, 3, 0, 6, 99], opcodes);
        }
        {
            let mut opcodes: Vec<u32> = vec![2, 4, 4, 5, 99, 0];
            processor(&mut opcodes);
            assert_eq!(vec![2, 4, 4, 5, 99, 9801], opcodes);
        }
        {
            let mut opcodes: Vec<u32> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
            processor(&mut opcodes);
            assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], opcodes);
        }
    }
}
