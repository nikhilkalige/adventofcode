use std::io::{self};

fn main() -> io::Result<()> {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/input.txt"));
    let mut opcodes: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().expect("Input should be a number"))
        .collect();

    let sim_stdin = Some(1);
    let result = processor(&mut opcodes, sim_stdin);
    println!("{:?}", result);
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
