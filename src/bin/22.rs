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

pub fn part_one(input: &str) -> Option<u128> {
    let numbers: Vec<u128> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();

    Some(
        numbers
            .into_iter()
            .map(|number| calculate_nth(number, 2000))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u128> {
    None
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
        assert_eq!(calculate_nth(1, 2000), 8685429);
        assert_eq!(calculate_nth(10, 2000), 4700978);
        assert_eq!(calculate_nth(100, 2000), 15273692);
        assert_eq!(calculate_nth(2024, 2000), 8667524);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
