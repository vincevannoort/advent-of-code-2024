use std::iter::repeat;

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord)]
enum Operator {
    Plus,
    Times,
    Concatenation,
}

pub fn part_one(input: &str) -> Option<u64> {
    let result: u64 = input
        .lines()
        .flat_map(|line| {
            let (test_value, numbers) = line.split_once(": ").unwrap();
            let test_value: u64 = test_value.parse().unwrap();
            let numbers: Vec<u64> = numbers
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect_vec();

            let operator_combinations = repeat([Operator::Plus, Operator::Times])
                .take(numbers.len() - 1)
                .multi_cartesian_product()
                .collect_vec();

            let first_number = numbers.first().unwrap();

            let valid_option = operator_combinations.into_iter().find(|operators| {
                let result = numbers.iter().skip(1).zip(operators).fold(
                    *first_number,
                    |acc, (value, operator)| match operator {
                        Operator::Plus => acc + value,
                        Operator::Times => acc * value,
                        _ => panic!(),
                    },
                );

                result == test_value
            });

            if valid_option.is_some() {
                Some(test_value)
            } else {
                None
            }
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result: u64 = input
        .lines()
        .flat_map(|line| {
            let (test_value, numbers) = line.split_once(": ").unwrap();
            let test_value: u64 = test_value.parse().unwrap();
            let numbers: Vec<u64> = numbers
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect_vec();

            let operator_combinations =
                repeat([Operator::Plus, Operator::Times, Operator::Concatenation])
                    .take(numbers.len() - 1)
                    .multi_cartesian_product()
                    .collect_vec();

            let first_number = numbers.first().unwrap();

            let valid_option = operator_combinations.into_iter().find(|operators| {
                let result = numbers.iter().skip(1).zip(operators).fold(
                    *first_number,
                    |acc, (value, operator)| match operator {
                        Operator::Plus => acc + value,
                        Operator::Times => acc * value,
                        Operator::Concatenation => {
                            (acc.to_string() + &value.to_string()).parse().unwrap()
                        }
                    },
                );

                result == test_value
            });

            if valid_option.is_some() {
                Some(test_value)
            } else {
                None
            }
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
