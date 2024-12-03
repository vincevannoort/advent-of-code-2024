use regex::{Captures, Regex};
advent_of_code::solution!(3);

fn get_multiplied_value_from_capture(c: Captures<'_>) -> u32 {
    let left = &c.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let right = &c.get(2).unwrap().as_str().parse::<u32>().unwrap();
    left * right
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let total = re
        .captures_iter(input)
        .map(|c| get_multiplied_value_from_capture(c))
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let total = re
        .captures_iter(input)
        .fold((0, true), |(total, enabled), c| {
            match c.get(0).unwrap().as_str() {
                "do()" => (total, true),
                "don't()" => (total, false),
                _ => (
                    match enabled {
                        true => total + get_multiplied_value_from_capture(c),
                        false => total,
                    },
                    enabled,
                ),
            }
        })
        .0;

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
