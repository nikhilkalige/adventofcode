use std::io::{self};

fn main() -> io::Result<()> {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/input.txt"));
    let opcodes: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().expect("Input should be a number"))
        .collect();

    {
        let mut opcodes = opcodes.clone();
        let result = processor(&mut opcodes, Some(1));
        println!("{:?}", result);
    }
    {
        let mut opcodes = opcodes.clone();
        let result = processor(&mut opcodes, Some(5));
        println!("{:?}", result);
    }
    Ok(())
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: i32,
    modes: Vec<Mode>,
}

impl Instruction {
    fn mode(&self, parameter: usize) -> Mode {
        self.modes.get(parameter).copied().unwrap_or(Mode::Position)
    }
}

fn instruction(mut opcode: i32) -> Instruction {
    let op = opcode % 100;
    opcode /= 100;

    let mut modes = vec![];
    while opcode > 0 {
        let mode = match opcode % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            other => panic!("Invalid parameter mode {}", other),
        };
        modes.push(mode);
        opcode /= 10;
    }
    Instruction { opcode: op, modes }
}

fn processor(opcodes: &mut [i32], input: Option<i32>) -> Vec<i32> {
    let mut index: usize = 0;
    let mut current_inst = instruction(opcodes[index]);
    let mut output = vec![];

    while current_inst.opcode != 99 {
        match current_inst.opcode {
            1 => {
                let result_location = opcodes[index + 3] as usize;
                let a = get_value(opcodes, &current_inst, index, 0);
                let b = get_value(opcodes, &current_inst, index, 1);
                let ans = a + b;
                opcodes[result_location] = ans;
                index += 4;
            }
            2 => {
                let result_location = opcodes[index + 3] as usize;
                let a = get_value(opcodes, &current_inst, index, 0);
                let b = get_value(opcodes, &current_inst, index, 1);
                let ans = a * b;
                opcodes[result_location] = ans;
                index += 4;
            }
            3 => {
                let result_location = opcodes[index + 1] as usize;
                opcodes[result_location] = input.expect("Expected input for opcode 3");
                index += 2;
            }
            4 => {
                let result = get_value(opcodes, &current_inst, index, 0);
                output.push(result);
                index += 2;
            }
            5 => {
                let test_value = get_value(opcodes, &current_inst, index, 0);
                if test_value != 0 {
                    let jump_position = get_value(opcodes, &current_inst, index, 1);
                    index = jump_position as usize;
                } else {
                    index += 3;
                }
            }
            6 => {
                let test_value = get_value(opcodes, &current_inst, index, 0);
                if test_value == 0 {
                    let jump_position = get_value(opcodes, &current_inst, index, 1);
                    index = jump_position as usize;
                } else {
                    index += 3;
                }
            }
            7 => {
                let position = opcodes[index + 3] as usize;
                let pa = get_value(opcodes, &current_inst, index, 0);
                let pb = get_value(opcodes, &current_inst, index, 1);
                let answer = if pa < pb { 1 } else { 0 };
                opcodes[position] = answer;
                index += 4;
            }
            8 => {
                let position = opcodes[index + 3] as usize;
                let pa = get_value(opcodes, &current_inst, index, 0);
                let pb = get_value(opcodes, &current_inst, index, 1);
                let answer = if pa == pb { 1 } else { 0 };
                opcodes[position] = answer;
                index += 4;
            }
            other => panic!("Unknown opcode {}", other),
        }
        current_inst = instruction(opcodes[index]);
    }

    output
}

fn get_value(opcodes: &[i32], inst: &Instruction, index: usize, parameter_index: usize) -> i32 {
    let param_location = index + parameter_index + 1;
    match inst.mode(parameter_index) {
        Mode::Position => opcodes[opcodes[param_location] as usize],
        Mode::Immediate => opcodes[param_location],
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_processor() {
        let mut input1: Vec<i32> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(processor(&mut input1, Some(8)), vec![1]);

        let mut input2: Vec<i32> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(processor(&mut input2, Some(0)), vec![0]);
        assert_eq!(processor(&mut input2, Some(1)), vec![1]);

        let mut input3: Vec<i32> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(processor(&mut input3, Some(0)), vec![0]);
        // assert_eq!(processor(&mut input3, Some(1)), vec![1]);

        let mut input4: Vec<i32> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(processor(&mut input4, Some(7)), vec![999]);
        assert_eq!(processor(&mut input4, Some(8)), vec![1000]);
        assert_eq!(processor(&mut input4, Some(9)), vec![1001]);
    }
}
