use std::convert::TryInto;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let numbers = input.lines().map(|s| s.parse::<u32>().unwrap());
    let fuel_data = numbers.map(|n| (fuel_calculator(n), recursive_fuel_calculator(n)));

    let mut simple_total: u32 = 0;
    let mut recursive_total: u32 = 0;
    for (simple, recursive) in fuel_data {
        simple_total += simple;
        recursive_total += recursive;
    }

    println!("Mass is {}, {}", simple_total, recursive_total);
    Ok(())
}

fn fuel_calculator(mass: u32) -> u32 {
    let x = mass / 3;
    let y: i32 = x as i32 - 2;
    if y > 0 {
        y.try_into().unwrap()
    } else {
        0
    }
}

fn recursive_fuel_calculator(mass: u32) -> u32 {
    let fuel = fuel_calculator(mass);
    if fuel == 0 {
        fuel
    } else {
        fuel + recursive_fuel_calculator(fuel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_calculator() {
        assert_eq!(2, fuel_calculator(12));
        assert_eq!(2, fuel_calculator(14));
        assert_eq!(654, fuel_calculator(1969));
        assert_eq!(33583, fuel_calculator(100756));
    }

    #[test]
    fn test_recursive_fuel_calculator() {
        assert_eq!(2, recursive_fuel_calculator(14));
        assert_eq!(966, recursive_fuel_calculator(1969));
        assert_eq!(50346, recursive_fuel_calculator(100756));
    }
}
