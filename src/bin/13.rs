use advent_of_code::Location;
use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(13);

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord)]
struct Button {
    x: u32,
    y: u32,
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord)]
struct Machine {
    a: Button,
    b: Button,
    price: Location,
}

fn parse_button(button: &str) -> Button {
    let re = Regex::new(r"X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let caps = re.captures(button).unwrap();
    Button {
        x: caps["x"].parse().unwrap(),
        y: caps["y"].parse().unwrap(),
    }
}

fn parse_machine(a: &str, b: &str, c: &str) -> Machine {
    let re = Regex::new(r"X\=(?<x>\d+), Y\=(?<y>\d+)").unwrap();
    let caps = re.captures(c).unwrap();
    Machine {
        a: parse_button(a),
        b: parse_button(b),
        price: Location {
            x: caps["x"].parse().unwrap(),
            y: caps["y"].parse().unwrap(),
        },
    }
}

fn try_combinations(machine: &Machine) -> Option<u32> {
    let max_a_x = machine.price.x / machine.a.x;
    let max_a_y = machine.price.y / machine.a.y;
    let max_b_x = machine.price.x / machine.b.x;
    let max_b_y = machine.price.y / machine.b.y;
    let max_x = 100.min(max_a_x.max(max_b_x));
    let max_y = 100.min(max_a_y.max(max_b_y));

    let results = (0..=max_x)
        .flat_map(|a_presses| {
            (0..=max_y)
                .flat_map(|b_presses| {
                    let x = a_presses * machine.a.x + b_presses * machine.b.x;
                    let y = a_presses * machine.a.y + b_presses * machine.b.y;
                    let end_location = Location { x, y };
                    if end_location == machine.price {
                        Some(a_presses * 3 + b_presses)
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let minimum = results.iter().min();
    minimum.cloned()
}

pub fn part_one(input: &str) -> Option<u32> {
    let result: u32 = input
        .lines()
        .filter(|l| !l.is_empty())
        .chunks(3)
        .into_iter()
        .filter_map(|chunk| {
            let (a, b, c) = chunk.into_iter().collect_tuple().unwrap();
            let machine = parse_machine(a, b, c);
            try_combinations(&machine)
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
