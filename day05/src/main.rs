use intcode::simple_processor;
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
        let result = simple_processor(&mut opcodes, vec![1]);
        println!("{:?}", result);
    }
    {
        let mut opcodes = opcodes.clone();
        let result = simple_processor(&mut opcodes, vec![5]);
        println!("{:?}", result);
    }
    Ok(())
}
