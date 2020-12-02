use intcode::simple_processor;
use std::io::{self};

fn main() -> io::Result<()> {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/input.txt"));
    let opcodes: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| s.parse().expect("Input should be a number"))
        .collect();

    {
        let opcodes = opcodes.clone();
        let (_, result) = simple_processor(&opcodes, vec![1]);
        println!("{:?}", result);
    }
    {
        let opcodes = opcodes.clone();
        let (_, result) = simple_processor(&opcodes, vec![2]);
        println!("{:?}", result);
    }
    Ok(())
}
