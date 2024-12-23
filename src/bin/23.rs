use itertools::{self, Itertools};
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let connections: HashMap<&str, HashSet<&str>> = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('-').unwrap();
            (left, right)
        })
        .fold(HashMap::new(), |mut acc, (left, right)| {
            // left -> right
            acc.entry(left)
                .and_modify(|entry| {
                    entry.insert(right);
                })
                .or_insert(HashSet::from_iter([right]));
            // right -> left
            acc.entry(right)
                .and_modify(|entry| {
                    entry.insert(left);
                })
                .or_insert(HashSet::from_iter([left]));
            acc
        });

    let combinations = connections.keys().combinations(3).collect_vec();

    let inter_connected_computers = combinations
        .iter()
        .filter(|combination| {
            let (a, b, c) = combination.iter().collect_tuple().unwrap();

            // one should start with t
            if !(a.starts_with('t') || b.starts_with('t') || c.starts_with('t')) {
                return false;
            }

            let a_conn = connections
                .get(*a)
                .map(|a| a.contains(*b) && a.contains(*c))
                .unwrap_or_default();
            let b_conn = connections
                .get(*b)
                .map(|b| b.contains(*a) && b.contains(*c))
                .unwrap_or_default();
            let c_conn = connections
                .get(*c)
                .map(|c| c.contains(*a) && c.contains(*b))
                .unwrap_or_default();
            a_conn && b_conn && c_conn
        })
        .count();

    Some(inter_connected_computers.try_into().unwrap())
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
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
