use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(22);

fn prune(a: u128) -> u128 {
    a.rem_euclid(16777216)
}

fn mix(a: u128, b: u128) -> u128 {
    a ^ b
}

fn calculate(secret: u128) -> u128 {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    prune(mix(secret, secret * 2048))
}

fn calculate_nth(secret: u128, nth: u32) -> u128 {
    let mut secret = secret;
    for i in 0..nth {
        secret = calculate(secret);
    }
    secret
}

fn calculate_consecutive(secret: u128, consecutive: u32) -> Vec<u128> {
    let mut secrets = vec![secret];
    for i in 0..(consecutive - 1) {
        secrets.push(calculate_nth(*secrets.last().unwrap(), 1));
    }
    secrets
}

fn calculate_consecutive_prices(secret: u128, consecutive: u32) -> Vec<u128> {
    calculate_consecutive(secret, consecutive)
        .iter()
        .map(|secret| {
            secret
                .to_string()
                .chars()
                .last()
                .unwrap()
                .to_string()
                .parse()
                .unwrap()
        })
        .collect_vec()
}

fn calculate_consecutive_price_changes(secret: u128, consecutive: u32) -> Vec<i128> {
    calculate_consecutive_prices(secret, consecutive)
        .iter()
        .tuple_windows()
        .map(|(a, b)| *b as i128 - *a as i128)
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u128> {
    let numbers: Vec<u128> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();

    Some(
        numbers
            .into_iter()
            .map(|number| calculate_nth(number, 1))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u128> {
    const AMOUNT: u32 = 2000;
    let numbers: Vec<u128> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();

    let buyers_with_price_and_changes = numbers
        .clone()
        .into_iter()
        .map(|number| {
            let prices = calculate_consecutive_prices(number, AMOUNT + 1)[1..].to_vec();
            let price_changes = calculate_consecutive_price_changes(number, AMOUNT + 1);
            (prices, price_changes)
        })
        .collect_vec();

    let price_change_sequences: HashSet<&[i128]> = HashSet::from_iter(
        buyers_with_price_and_changes
            .iter()
            .flat_map(|(_, changes)| changes.windows(4).collect_vec())
            .collect_vec(),
    );

    let max_price_change = price_change_sequences
        .iter()
        // for each price change option, we check whether the change occurs
        .map(|price_change_sequence| {
            let total_price: u128 = buyers_with_price_and_changes
                .iter()
                .map(|(prices, price_changes)| {
                    // does the sequence occur anywhere
                    let Some((price_change_pos, _)) = price_changes
                        .windows(4)
                        .find_position(|price_change| price_change == price_change_sequence)
                    else {
                        return 0;
                    };

                    // the price at the end of our changes
                    let price_change_pos = price_change_pos + 3;
                    prices[price_change_pos]
                })
                .sum();

            (total_price, price_change_sequence)
        })
        .max_by_key(|(total_price, _)| *total_price)
        .unwrap();

    Some(max_price_change.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(123), 15887950);
        assert_eq!(calculate(15887950), 16495136);
    }

    #[test]
    fn test_calculate_nth() {
        assert_eq!(calculate_nth(123, 1), 15887950);
        assert_eq!(calculate_nth(1, 2000), 8685429);
        assert_eq!(calculate_nth(10, 2000), 4700978);
        assert_eq!(calculate_nth(100, 2000), 15273692);
        assert_eq!(calculate_nth(2024, 2000), 8667524);
    }

    #[test]
    fn test_calculate_consecutive() {
        assert_eq!(calculate_consecutive(123, 3), [123, 15887950, 16495136]);
    }

    #[test]
    fn test_calculate_consecutive_prices() {
        assert_eq!(calculate_consecutive_prices(123, 3), [3, 0, 6]);
    }

    #[test]
    fn test_calculate_consecutive_price_changes() {
        assert_eq!(calculate_consecutive_price_changes(123, 3), [-3, 6]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
