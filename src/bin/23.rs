use itertools::{self, Itertools};
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

pub fn parse_connections(input: &str) -> HashMap<&str, HashSet<&str>> {
    input
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
        })
}

fn get_combination<'a>(
    connections: &'a HashMap<&'a str, HashSet<&'a str>>,
    n: u32,
) -> Vec<Vec<&'a &'a str>> {
    connections
        .keys()
        .combinations(n.try_into().unwrap())
        // prune, all these should atleast be connnected to n - 1 computers
        .filter(|combination| {
            combination.iter().all(|computer| {
                connections.get(*computer).unwrap().len() >= (n - 1).try_into().unwrap()
            })
        })
        .collect_vec()
}

fn check_inter_connected<'a>(
    connections: &'a HashMap<&'a str, HashSet<&'a str>>,
    combination: &&Vec<&&str>,
    should_start_with_t: bool,
) -> bool {
    if should_start_with_t && !combination.iter().any(|computer| computer.starts_with('t')) {
        return false;
    }

    combination.iter().all(|computer| {
        let computer_connections = connections.get(*computer).unwrap();

        combination.iter().all(|other_computer| {
            if computer == other_computer {
                return true;
            }
            computer_connections.contains(*other_computer)
        })
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let connections: HashMap<&str, HashSet<&str>> = parse_connections(input);
    let combinations: Vec<Vec<&&str>> = get_combination(&connections, 3);

    let inter_connected_computers = combinations
        .iter()
        .filter(|combination| check_inter_connected(&connections, combination, true))
        .count();

    Some(inter_connected_computers.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<String> {
    let connections: HashMap<&str, HashSet<&str>> = parse_connections(input);

    let max_connections = connections
        .iter()
        .map(|(computer, other_computers)| {
            let mut other_computers = other_computers.iter().cloned().collect_vec();

            let mut subset: HashSet<&str> = HashSet::new();
            subset.insert(computer);

            loop {
                let other_computer = other_computers.pop();
                let Some(other_computer) = other_computer else {
                    break;
                };

                let other_computer_connections = connections.get(other_computer).unwrap();

                // if it is not connected, we skip adding it to the subset
                if !other_computer_connections.is_superset(&subset) {
                    continue;
                }

                subset.insert(other_computer);
            }
            subset
        })
        .max_by_key(|subset| subset.len());

    Some(max_connections.unwrap().iter().sorted().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
