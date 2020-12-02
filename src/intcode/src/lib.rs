use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, Sender},
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: i64,
    modes: Vec<Mode>,
}

impl Instruction {
    fn mode(&self, parameter: usize) -> Mode {
        self.modes.get(parameter).copied().unwrap_or(Mode::Position)
    }
}

fn instruction(mut opcode: i64) -> Instruction {
    let op = opcode % 100;
    opcode /= 100;

    let mut modes = vec![];
    while opcode > 0 {
        let mode = match opcode % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            other => panic!("Invalid parameter mode {}", other),
        };
        modes.push(mode);
        opcode /= 10;
    }
    Instruction { opcode: op, modes }
}

struct Computer {
    program: HashMap<usize, i64>,
    current_position: usize,
    relative_base: usize,
}

impl Computer {
    fn current_instruction(&self) -> Instruction {
        instruction(self.read_at(self.current_position))
    }

    fn get_value(&self, parameter_index: usize) -> i64 {
        get_value(
            &self.program,
            &self.current_instruction(),
            self.current_position,
            parameter_index,
            self.relative_base,
        )
    }

    fn set_value(&mut self, parameter_index: usize, value: i64) {
        let instruction = self.current_instruction();
        set_value(
            &mut self.program,
            self.current_position,
            &instruction,
            parameter_index,
            self.relative_base,
            value,
        );
    }

    fn read_at(&self, index: usize) -> i64 {
        self.program.get(&index).copied().unwrap_or(0)
    }
}

pub fn simple_processor(opcodes: &[i64], inputs: Vec<i64>) -> (HashMap<usize, i64>, Vec<i64>) {
    let (main_sender, processor_reciever) = channel();
    let (processor_sender, main_reciever) = channel();

    for input in inputs.into_iter() {
        main_sender.send(input).unwrap();
    }

    drop(main_sender);
    let opcodes = processor(opcodes, processor_reciever, processor_sender);

    let mut outputs = Vec::<i64>::new();
    while let Ok(output) = main_reciever.recv() {
        outputs.push(output);
    }
    (opcodes, outputs)
}

pub fn processor(
    opcodes: &[i64],
    inputs: Receiver<i64>,
    outputs: Sender<i64>,
) -> HashMap<usize, i64> {
    let program: HashMap<usize, i64> = opcodes.iter().cloned().enumerate().collect();

    let mut computer = Computer {
        program,
        current_position: 0,
        relative_base: 0,
    };

    let mut current_inst = computer.current_instruction();

    while current_inst.opcode != 99 {
        match current_inst.opcode {
            1 => {
                let a = computer.get_value(0);
                let b = computer.get_value(1);
                let ans = a + b;
                computer.set_value(2, ans);
                computer.current_position += 4;
            }
            2 => {
                let a = computer.get_value(0);
                let b = computer.get_value(1);
                let ans = a * b;
                computer.set_value(2, ans);
                computer.current_position += 4;
            }
            3 => {
                let input = inputs
                    .recv()
                    .expect("Should have had enough inputs for opcode 3");
                computer.set_value(0, input);
                computer.current_position += 2;
            }
            4 => {
                let result = computer.get_value(0);
                outputs.send(result).unwrap();
                computer.current_position += 2;
            }
            5 => {
                let test_value = computer.get_value(0);
                if test_value != 0 {
                    let jump_position = computer.get_value(1);
                    computer.current_position = jump_position as usize;
                } else {
                    computer.current_position += 3;
                }
            }
            6 => {
                let test_value = computer.get_value(0);
                if test_value == 0 {
                    let jump_position = computer.get_value(1);
                    computer.current_position = jump_position as usize;
                } else {
                    computer.current_position += 3;
                }
            }
            7 => {
                let pa = computer.get_value(0);
                let pb = computer.get_value(1);
                let answer = if pa < pb { 1 } else { 0 };
                computer.set_value(2, answer);
                computer.current_position += 4;
            }
            8 => {
                let pa = computer.get_value(0);
                let pb = computer.get_value(1);
                let answer = if pa == pb { 1 } else { 0 };
                computer.set_value(2, answer);
                computer.current_position += 4;
            }
            9 => {
                let offset = computer.get_value(0);
                computer.relative_base = (computer.relative_base as i64 + offset) as usize;
                computer.current_position += 2;
            }
            other => panic!("Unknown opcode {}", other),
        }
        current_inst = computer.current_instruction();
    }
    computer.program
}

fn get_value(
    opcodes: &HashMap<usize, i64>,
    inst: &Instruction,
    index: usize,
    parameter_index: usize,
    relative_base: usize,
) -> i64 {
    let param_location = index + parameter_index + 1;
    match inst.mode(parameter_index) {
        Mode::Position => {
            let position = opcodes.get(&param_location).copied().unwrap_or(0) as usize;
            opcodes.get(&position).copied().unwrap_or(0)
        }
        Mode::Immediate => opcodes.get(&param_location).copied().unwrap_or(0),
        Mode::Relative => {
            let offset = opcodes.get(&param_location).copied().unwrap_or(0);
            let position = offset + relative_base as i64;
            opcodes.get(&(position as usize)).copied().unwrap_or(0)
        }
    }
}

fn set_value(
    program: &mut HashMap<usize, i64>,
    instruction_pointer: usize,
    inst: &Instruction,
    parameter_index: usize,
    relative_base: usize,
    value: i64,
) {
    let param_location = instruction_pointer + parameter_index + 1;
    match inst.mode(parameter_index) {
        Mode::Position => {
            let position = program.get(&param_location).copied().unwrap_or(0) as usize;
            program.insert(position, value);
        }
        Mode::Immediate => unreachable!("Can't set values in immediate mode"),
        Mode::Relative => {
            let offset = program.get(&param_location).copied().unwrap_or(0);
            let position = offset + relative_base as i64;
            if position < 0 {
                panic!("Cannot access memory at {}", position);
            }
            program.insert(position as usize, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn opcode_99_ends() {
        let program = vec![99];
        let (program, _) = simple_processor(&program, vec![]);
        assert_eq!(program, vec![99].into_iter().enumerate().collect());
    }

    #[test]
    fn opcode_1_adds() {
        let program = vec![1, 0, 0, 0, 99];
        let (program, _) = simple_processor(&program, vec![]);
        assert_eq!(
            program,
            vec![2, 0, 0, 0, 99].into_iter().enumerate().collect()
        );
    }

    #[test]
    fn opcode_2_multiplies() {
        let program = vec![2, 3, 0, 3, 99];
        let (program, _) = simple_processor(&program, vec![]);
        assert_eq!(
            program,
            vec![2, 3, 0, 6, 99].into_iter().enumerate().collect()
        );
    }

    #[test]
    fn multiply_and_store_after_program() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let (program, _) = simple_processor(&program, vec![]);
        assert_eq!(
            program,
            vec![2, 4, 4, 5, 99, 9801].into_iter().enumerate().collect()
        );
    }

    #[test]
    fn program_keeps_going_if_an_instruction_changes() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let (program, _) = simple_processor(&program, vec![]);
        assert_eq!(
            program,
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
                .into_iter()
                .enumerate()
                .collect()
        );
    }

    #[test]
    fn opcode_3_takes_input() {
        let program = vec![3, 0, 99];
        let (program, _) = simple_processor(&program, vec![7]);
        assert_eq!(program, vec![7, 0, 99].into_iter().enumerate().collect());
    }

    #[test]
    fn programs_can_have_arbitrary_numbers_of_opcode_3_with_enough_input() {
        let program = vec![3, 0, 3, 1, 99];
        let (program, _) = simple_processor(&program, vec![7, 8]);
        assert_eq!(
            program,
            vec![7, 8, 3, 1, 99].into_iter().enumerate().collect()
        );
    }

    #[test]
    #[should_panic(expected = "Should have had enough inputs for opcode 3")]
    fn programs_panics_if_opcode_3_doesnt_have_input() {
        let program = vec![3, 0, 3, 1, 99];
        simple_processor(&program, vec![7]);
    }

    #[test]
    fn opcode_4_returns_output() {
        let program = vec![4, 2, 99];
        let (_, output) = simple_processor(&program, vec![]);
        assert_eq!(output, vec![99]);
    }

    #[test]
    fn opcode_5_jumps_if_true() {
        // Test value is false; 42 gets printed
        let program = vec![1005, 6, 5, 104, 42, 99, 0];
        let (_, output) = simple_processor(&program, vec![]);
        assert_eq!(output, vec![42]);

        // Test value is true; print gets jumped over
        let program = vec![1005, 6, 5, 104, 42, 99, 3];
        let (_, output) = simple_processor(&program, vec![]);
        assert_eq!(output, vec![]);
    }

    #[test]
    fn opcode_6_jumps_if_false() {
        // Test value is false; print gets jumped over
        let program = vec![1006, 6, 5, 104, 42, 99, 0];
        let (_, output) = simple_processor(&program, vec![]);
        assert_eq!(output, vec![]);

        // Test value is true; 42 gets printed
        let program = vec![1006, 6, 5, 104, 42, 99, 3];
        let (_, output) = simple_processor(&program, vec![]);
        assert_eq!(output, vec![42]);
    }

    #[test]
    fn opcode_7_less_than() {
        let program = vec![1107, 4, 5, 3, 99];
        let (program, _) = simple_processor(&program, vec![]);
        assert_eq!(
            program,
            vec![1107, 4, 5, 1, 99].into_iter().enumerate().collect()
        );

        let program = vec![1107, 5, 4, 3, 99];
        let (program, _) = simple_processor(&program, vec![]);
        assert_eq!(
            program,
            vec![1107, 5, 4, 0, 99].into_iter().enumerate().collect()
        );
    }

    #[test]
    fn opcode_8_equals() {
        let program = vec![1108, 4, 4, 3, 99];
        let (program, _) = simple_processor(&program, vec![]);
        assert_eq!(
            program,
            vec![1108, 4, 4, 1, 99].into_iter().enumerate().collect()
        );

        let program = vec![1108, 5, 4, 3, 99];
        let (program, _) = simple_processor(&program, vec![]);
        assert_eq!(
            program,
            vec![1108, 5, 4, 0, 99].into_iter().enumerate().collect()
        );
    }

    #[test]
    fn opcode_9_relative_offset() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let (_, output) = simple_processor(&program, vec![]);
        assert_eq!(output, vec![1219070632396864]);

        let (_, output) = simple_processor(&mut vec![104, 1125899906842624, 99], vec![]);
        assert_eq!(output, vec![1125899906842624]);
    }

    #[test]
    fn test_processor() {
        let input1: Vec<i64> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(simple_processor(&input1, vec![8]).1, vec![1]);

        let input2: Vec<i64> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(simple_processor(&input2, vec![0]).1, vec![0]);
        assert_eq!(simple_processor(&input2, vec![1]).1, vec![1]);

        let input3: Vec<i64> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(simple_processor(&input3, vec![0]).1, vec![0]);

        let input4: Vec<i64> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(simple_processor(&input4, vec![7]).1, vec![999]);
        assert_eq!(simple_processor(&input4, vec![8]).1, vec![1000]);
        assert_eq!(simple_processor(&input4, vec![9]).1, vec![1001]);

        let (_, output) = simple_processor(
            &vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
            vec![],
        );
        assert_eq!(
            output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }
}
