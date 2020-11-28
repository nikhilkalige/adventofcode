use intcode::{processor, simple_processor};
use itertools::Itertools;
use std::io::{self};
use std::sync::mpsc::channel;
use std::thread;

fn main() -> io::Result<()> {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/input.txt"));
    let opcodes: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().expect("Input should be a number"))
        .collect();

    let phase_settings = vec![0, 1, 2, 3, 4];
    println!("Max signal {}", max_signal(&opcodes, &phase_settings));

    let phase_settings: Vec<_> = (5..=9).collect();
    println!(
        "Max signal with feedback {}",
        max_signal_with_feedback(&opcodes, &phase_settings)
    );
    Ok(())
}

fn run_with_phase_setting(opcodes: &[i32], phase_settings: &[u32]) -> i32 {
    let mut input = 0;
    for &phase in phase_settings {
        let output = simple_processor(&mut opcodes.to_owned(), vec![phase as i32, input]);
        input = *output.first().expect("Program must return a value");
    }

    input
}

fn run_with_phase_setting_feedback(opcodes: &[i32], phase_settings: &[u32]) -> i32 {
    let (main_send, amp_a_recv) = channel();
    let (amp_a_send, amp_b_recv) = channel();
    let (amp_b_send, amp_c_recv) = channel();
    let (amp_c_send, amp_d_recv) = channel();
    let (amp_d_send, amp_e_recv) = channel();
    let (amp_e_send, main_recv) = channel();

    main_send.send(phase_settings[0] as i32).unwrap();
    amp_a_send.send(phase_settings[1] as i32).unwrap();
    amp_b_send.send(phase_settings[2] as i32).unwrap();
    amp_c_send.send(phase_settings[3] as i32).unwrap();
    amp_d_send.send(phase_settings[4] as i32).unwrap();

    let mut program = opcodes.to_vec();
    thread::spawn(move || {
        processor(&mut program, amp_a_recv, amp_a_send);
    });
    let mut program = opcodes.to_vec();
    thread::spawn(move || {
        processor(&mut program, amp_b_recv, amp_b_send);
    });
    let mut program = opcodes.to_vec();
    thread::spawn(move || {
        processor(&mut program, amp_c_recv, amp_c_send);
    });
    let mut program = opcodes.to_vec();
    thread::spawn(move || {
        processor(&mut program, amp_d_recv, amp_d_send);
    });
    let mut program = opcodes.to_vec();
    thread::spawn(move || {
        processor(&mut program, amp_e_recv, amp_e_send);
    });

    main_send.send(0).unwrap();
    let mut result = 0;
    while let Ok(output) = main_recv.recv() {
        let _ = main_send.send(output);
        result = output;
    }
    result
}

fn max_signal(opcodes: &[i32], phase_settings: &[u32]) -> i32 {
    let outputs = phase_settings.iter().permutations(5).map(|phase| {
        let phase_configuration: Vec<u32> = phase.iter().map(|v| **v).collect();
        let output = run_with_phase_setting(&opcodes.to_owned(), &phase_configuration);
        output
    });

    outputs.max().expect("Must have had orderings")
}

fn max_signal_with_feedback(opcodes: &[i32], phase_settings: &[u32]) -> i32 {
    let outputs = phase_settings.iter().permutations(5).map(|phase| {
        let phase_configuration: Vec<u32> = phase.iter().map(|v| **v).collect();
        let output = run_with_phase_setting_feedback(&opcodes.to_owned(), &phase_configuration);
        output
    });

    outputs.max().expect("Must have had orderings")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_with_phase_setting() {
        let program = vec![3, 11, 3, 12, 1, 11, 12, 13, 4, 13, 99, -1, -2, -3];
        let phase_settings = [20, 22, 24];
        assert_eq!(run_with_phase_setting(&program, &phase_settings), 66);

        let phase_settings = [4, 3, 2, 1, 0];
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(run_with_phase_setting(&program, &phase_settings), 43210);

        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phase_settings = [0, 1, 2, 3, 4];
        assert_eq!(run_with_phase_setting(&program, &phase_settings), 54321);

        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phase_settings = vec![1, 0, 4, 3, 2];
        assert_eq!(run_with_phase_setting(&program, &phase_settings), 65210);
    }

    #[test]
    fn test_max_signal() {
        let phase_settings = [0, 1, 2, 3, 4];
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(max_signal(&program, &phase_settings), 43210);
    }

    #[test]
    fn test_run_with_phase_setting_feedback() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let phase_settings = [9, 8, 7, 6, 5];
        assert_eq!(
            run_with_phase_setting_feedback(&program, &phase_settings),
            139629729
        );

        let program = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let phase_settings = [9, 7, 8, 5, 6];
        assert_eq!(
            run_with_phase_setting_feedback(&program, &phase_settings),
            18216
        );
    }
}
