use itertools::Itertools;
use lazy_static::lazy_static;
use std::{cmp::Reverse, collections::HashMap, sync::Mutex};

advent_of_code::solution!(19);

lazy_static! {
    static ref CACHED_PATTERNS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn is_pattern_possible(design: &str, patterns: &Vec<&str>) -> bool {
    if let Some(cached_pattern) = {
        // fetched cached pattern
        let lock = CACHED_PATTERNS.lock().unwrap();
        lock.get(design).cloned()
    } {
        let design_slice = &design[cached_pattern.len()..];
        if design_slice.is_empty() {
            return true;
        }

        return is_pattern_possible(design_slice, patterns);
    }

    // calculate possible patterns
    let possible_patterns = patterns
        .iter()
        .filter(|pattern| design.starts_with(*pattern))
        .collect_vec();

    possible_patterns.iter().any(|possible_pattern| {
        // insert cached pattern
        {
            let mut lock = CACHED_PATTERNS.lock().unwrap();
            lock.insert(design.to_string(), possible_pattern.to_string());
        }

        let design_slice = &design[possible_pattern.len()..];
        if design_slice.is_empty() {
            return true;
        }
        is_pattern_possible(design_slice, patterns)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns = patterns
        .split(", ")
        .sorted_by_key(|pattern| Reverse(pattern.len()))
        .collect_vec();
    let designs = designs.lines().collect_vec();

    let possible_designs = designs
        .iter()
        .map(|design| is_pattern_possible(design, &patterns) as u32)
        .sum();
    Some(possible_designs)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
