use std::{
    collections::HashMap,
    io::{self},
};

fn main() -> io::Result<()> {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/input.txt"));
    let astroid_map = AstroidMap::new(input);

    Ok(())
}

struct AstroidMap {
    width: usize,
    height: usize,
    astroids: HashMap<(usize, usize), usize>,
}

impl AstroidMap {
    fn new(input: &str) -> AstroidMap {
        let mut width = 0;
        let mut height = 0;
        let mut astroids = HashMap::new();

        for line in input.lines() {
            width = 0;

            for c in line.chars() {
                if c == '#' {
                    astroids.insert((width, height), 0);
                }
                width += 1;
            }
            height += 1;
        }

        AstroidMap {
            width,
            height,
            astroids,
        }
    }
}
