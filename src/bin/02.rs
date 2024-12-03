use std::vec::IntoIter;

use itertools::{Itertools, TupleWindows};
advent_of_code::solution!(2);

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

fn is_valid_pair(pair: (u32, u32), direction: Direction) -> bool {
    if direction == Direction::Increasing && pair.0 > pair.1 {
        return false;
    };

    if direction == Direction::Decreasing && pair.0 < pair.1 {
        return false;
    };

    let difference = pair.0.abs_diff(pair.1);
    (1..=3).contains(&difference)
}

fn is_valid_pair_sequence(mut pairs: TupleWindows<IntoIter<u32>, (u32, u32)>) -> bool {
    let first_pair: (u32, u32) = pairs.clone().next().unwrap();

    let direction = if first_pair.0 > first_pair.1 {
        Direction::Decreasing
    } else {
        Direction::Increasing
    };

    pairs.all(|pair| is_valid_pair(pair, direction))
}

pub fn part_one(input: &str) -> Option<u32> {
    let safe_reports = input
        .lines()
        .map(|line| {
            // parse to numbers
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|number| number.trim().parse::<u32>().unwrap())
                .collect();

            // create pairs using numbers
            let pairs = numbers.into_iter().tuple_windows();

            is_valid_pair_sequence(pairs)
        })
        .map(|valid| valid as u32)
        .sum::<u32>();

    Some(safe_reports)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
