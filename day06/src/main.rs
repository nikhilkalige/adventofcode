use std::{
    collections::HashMap,
    io::{self},
};

fn main() -> io::Result<()> {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/input.txt"));
    let orbits = parse_input(input);

    // println!("{:?}", orbits);
    let num_orbits = count_orbits(&orbits);
    println!("Count: {}", num_orbits);

    let num_moves = num_transfers_to_santa(&orbits);
    println!("Moves: {}", num_moves);

    Ok(())
}

fn parse_input(input: &str) -> HashMap<&str, &str> {
    let mut orbits = HashMap::new();
    for line in input.lines() {
        let mut bodies = line.trim().split(')');
        let orbited = bodies.next().expect("Expected output of format A)B");
        let orbiting = bodies.next().expect("Expected output of format A)B");
        orbits.insert(orbiting, orbited);
    }
    orbits
}

fn count_orbits(orbits: &HashMap<&str, &str>) -> u32 {
    let counts = orbits
        .keys()
        .map(|child| get_orbit(child, orbits).len() as u32);
    counts.sum()
}

fn get_orbit(child: &str, orbits: &HashMap<&str, &str>) -> Vec<String> {
    let mut orbit = Vec::new();
    let mut curr_child = child;
    while orbits.contains_key(curr_child) {
        curr_child = orbits.get(curr_child).unwrap();
        orbit.push(curr_child.to_owned());
    }
    orbit
}

fn num_transfers_to_santa(orbits: &HashMap<&str, &str>) -> u32 {
    let santa_orbiting = get_orbit("SAN", orbits);
    let you_orbiting = get_orbit("YOU", orbits);

    let mut num_moves: u32 = 0;
    for (a, b) in you_orbiting.iter().rev().zip(santa_orbiting.iter().rev()) {
        if a != b {
            break;
        }
        num_moves += 1;
    }

    let num_elements = (santa_orbiting.len() + you_orbiting.len()) as u32;
    num_elements - 2 * num_moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_orbits() {
        const INPUT: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        assert_eq!(count_orbits(&parse_input(INPUT)), 42);
    }

    #[test]
    fn test_change_orbits() {
        const INPUT: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        assert_eq!(num_transfers_to_santa(&parse_input(INPUT)), 4);
    }
}
