use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

fn apply_rules(stone: u128) -> Vec<u128> {
    match stone {
        0 => vec![1],
        even if even.to_string().len() % 2 == 0 => {
            let test = even.to_string();
            let (left, right) = test.split_at(test.len() / 2);
            vec![left.parse().unwrap(), right.parse().unwrap()]
        }
        uneven => vec![uneven * 2024],
    }
}

fn blink_one(stones: Vec<u128>) -> Vec<u128> {
    stones.into_iter().flat_map(apply_rules).collect_vec()
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut stones: Vec<u128> = input
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();

    for _ in 0..25 {
        stones = blink_one(stones);
    }

    Some(stones.len() as u128)
}

fn blink_two(stones: HashMap<u128, u128>) -> HashMap<u128, u128> {
    let mut new_stones: HashMap<u128, u128> = HashMap::new();

    for (stone, count) in stones.into_iter() {
        for stone in apply_rules(stone) {
            *new_stones.entry(stone).or_insert(0) += count;
        }
    }

    new_stones
}

pub fn part_two(input: &str) -> Option<u128> {
    let stones: Vec<u128> = input
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();

    let mut stones: HashMap<u128, u128> =
        stones.into_iter().fold(HashMap::new(), |mut acc, key| {
            *acc.entry(key).or_insert(0) += 1;
            acc
        });

    for _ in 0..75 {
        stones = blink_two(stones);
    }

    Some(stones.into_values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(58330));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }

    #[test]
    fn test_part_two_simple() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(69680298005308));
    }
}
