use std::sync::mpsc::{channel, Receiver, Sender};

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

pub fn simple_processor(opcodes: &mut [i32], inputs: Vec<i32>) -> Vec<i32> {
    let (main_sender, processor_reciever) = channel();
    let (processor_sender, main_reciever) = channel();

    for input in inputs.into_iter() {
        main_sender.send(input).unwrap();
    }

    drop(main_sender);
    processor(opcodes, processor_reciever, processor_sender);

    let mut outputs = Vec::<i32>::new();
    while let Ok(output) = main_reciever.recv() {
        outputs.push(output);
    }
    outputs
}

pub fn processor(opcodes: &mut [i32], inputs: Receiver<i32>, outputs: Sender<i32>) {
    let mut index: usize = 0;
    let mut current_inst = instruction(opcodes[index]);

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
                opcodes[result_location] = inputs
                    .recv()
                    .expect("Should have had enough inputs for opcode 3");
                index += 2;
            }
            4 => {
                let result = get_value(opcodes, &current_inst, index, 0);
                outputs.send(result).unwrap();
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
    fn opcode_99_ends() {
        let mut program = vec![99];
        let _output = simple_processor(&mut program, vec![]);
        assert_eq!(program, vec![99]);
    }

    #[test]
    fn opcode_1_adds() {
        let mut program = vec![1, 0, 0, 0, 99];
        let _output = simple_processor(&mut program, vec![]);
        assert_eq!(program, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn opcode_2_multiplies() {
        let mut program = vec![2, 3, 0, 3, 99];
        let _output = simple_processor(&mut program, vec![]);
        assert_eq!(program, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn multiply_and_store_after_program() {
        let mut program = vec![2, 4, 4, 5, 99, 0];
        let _output = simple_processor(&mut program, vec![]);
        assert_eq!(program, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn program_keeps_going_if_an_instruction_changes() {
        let mut program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let _output = simple_processor(&mut program, vec![]);
        assert_eq!(program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn opcode_3_takes_input() {
        let mut program = vec![3, 0, 99];
        let _output = simple_processor(&mut program, vec![7]);
        assert_eq!(program, vec![7, 0, 99]);
    }

    #[test]
    fn programs_can_have_arbitrary_numbers_of_opcode_3_with_enough_input() {
        let mut program = vec![3, 0, 3, 1, 99];
        let _output = simple_processor(&mut program, vec![7, 8]);
        assert_eq!(program, vec![7, 8, 3, 1, 99]);
    }

    #[test]
    #[should_panic(expected = "Should have had enough inputs for opcode 3")]
    fn programs_panics_if_opcode_3_doesnt_have_input() {
        let mut program = vec![3, 0, 3, 1, 99];
        simple_processor(&mut program, vec![7]);
    }

    #[test]
    fn opcode_4_returns_output() {
        let mut program = vec![4, 2, 99];
        let output = simple_processor(&mut program, vec![]);
        assert_eq!(output, vec![99]);
    }

    #[test]
    fn opcode_5_jumps_if_true() {
        // Test value is false; 42 gets printed
        let mut program = vec![1005, 6, 5, 104, 42, 99, 0];
        let output = simple_processor(&mut program, vec![]);
        assert_eq!(output, vec![42]);

        // Test value is true; print gets jumped over
        let mut program = vec![1005, 6, 5, 104, 42, 99, 3];
        let output = simple_processor(&mut program, vec![]);
        assert_eq!(output, vec![]);
    }

    #[test]
    fn opcode_6_jumps_if_false() {
        // Test value is false; print gets jumped over
        let mut program = vec![1006, 6, 5, 104, 42, 99, 0];
        let output = simple_processor(&mut program, vec![]);
        assert_eq!(output, vec![]);

        // Test value is true; 42 gets printed
        let mut program = vec![1006, 6, 5, 104, 42, 99, 3];
        let output = simple_processor(&mut program, vec![]);
        assert_eq!(output, vec![42]);
    }

    #[test]
    fn opcode_7_less_than() {
        let mut program = vec![1107, 4, 5, 3, 99];
        let _output = simple_processor(&mut program, vec![]);
        assert_eq!(program, vec![1107, 4, 5, 1, 99]);

        let mut program = vec![1107, 5, 4, 3, 99];
        let _output = simple_processor(&mut program, vec![]);
        assert_eq!(program, vec![1107, 5, 4, 0, 99]);
    }

    #[test]
    fn opcode_8_equals() {
        let mut program = vec![1108, 4, 4, 3, 99];
        let _output = simple_processor(&mut program, vec![]);
        assert_eq!(program, vec![1108, 4, 4, 1, 99]);

        let mut program = vec![1108, 5, 4, 3, 99];
        let _output = simple_processor(&mut program, vec![]);
        assert_eq!(program, vec![1108, 5, 4, 0, 99]);
    }

    #[test]
    fn test_processor() {
        let mut input1: Vec<i32> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(simple_processor(&mut input1, vec![8]), vec![1]);

        let mut input2: Vec<i32> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(simple_processor(&mut input2, vec![0]), vec![0]);
        assert_eq!(simple_processor(&mut input2, vec![1]), vec![1]);

        let mut input3: Vec<i32> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(simple_processor(&mut input3, vec![0]), vec![0]);
        // assert_eq!(simple_processor(&mut input3, Some(1)), vec![1]);

        let mut input4: Vec<i32> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(simple_processor(&mut input4, vec![7]), vec![999]);
        assert_eq!(simple_processor(&mut input4, vec![8]), vec![1000]);
        assert_eq!(simple_processor(&mut input4, vec![9]), vec![1001]);
    }
}
