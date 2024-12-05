use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::str::Lines;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

advent_of_code::solution!(5);

type NumbersOrderedAfter = HashSet<u32>;

fn parse_page_ordering(line_iter: &mut Lines<'_>) -> HashMap<u32, NumbersOrderedAfter> {
    let mut page_ordering: HashMap<u32, NumbersOrderedAfter> = HashMap::new();

    line_iter
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let (left, right) = line.split_once('|').unwrap();
            let left = left.parse::<u32>().unwrap();
            let right = right.parse::<u32>().unwrap();
            page_ordering
                .entry(left)
                .and_modify(|entry| {
                    entry.insert(right);
                })
                .or_insert(NumbersOrderedAfter::from([right]));
        });

    page_ordering
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut line_iter = input.lines();

    let page_ordering: HashMap<u32, NumbersOrderedAfter> = parse_page_ordering(line_iter.by_ref());

    let result = line_iter
        // parse numbers
        .map(|pages_to_produce| {
            pages_to_produce
                .split(',')
                .map(|page| page.parse::<u32>().unwrap())
                .collect_vec()
        })
        .filter_map(|pages_to_produce| {
            let result = pages_to_produce.iter().fold_while(
                HashSet::<u32>::new(),
                |mut already_seen_numbers, value| {
                    if let Some(value) = page_ordering.get(value) {
                        if already_seen_numbers.intersection(value).count() > 0 {
                            return Done(already_seen_numbers);
                        }
                    }

                    already_seen_numbers.insert(*value);
                    Continue(already_seen_numbers)
                },
            );

            // ignore incorrectly sorted
            if let Done(_) = result {
                return None;
            }

            let middle = &pages_to_produce.len() / 2;
            let middle_page = &pages_to_produce.get(middle).unwrap();
            Some(**middle_page)
        })
        .sum::<u32>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut line_iter = input.lines();

    let page_ordering: HashMap<u32, NumbersOrderedAfter> = parse_page_ordering(line_iter.by_ref());

    let result = line_iter
        // parse numbers
        .map(|pages_to_produce| {
            pages_to_produce
                .split(',')
                .map(|page| page.parse::<u32>().unwrap())
                .collect_vec()
        })
        .filter_map(|pages_to_produce| {
            let sorted_pages_to_produce = pages_to_produce
                .clone()
                .into_iter()
                // sort by amount of numbers that have to go after it that are in the pages to produce
                .sorted_by_key(|page| {
                    let pages_required_after = match page_ordering.get(page) {
                        Some(values) => values
                            .intersection(&HashSet::from_iter(pages_to_produce.clone()))
                            .count(),
                        None => 0,
                    };
                    Reverse(pages_required_after)
                })
                .collect_vec();

            // ignore correctly sorted
            if pages_to_produce == sorted_pages_to_produce {
                return None;
            }

            let middle = &sorted_pages_to_produce.len() / 2;
            let middle_page = &sorted_pages_to_produce.get(middle).unwrap();
            Some(**middle_page)
        })
        .sum::<u32>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
