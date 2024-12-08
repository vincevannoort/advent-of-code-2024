use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use advent_of_code::{Grid, Location};

advent_of_code::solution!(8);

fn parse(input: &str) -> Grid<char> {
    let locations: HashMap<Location, char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Location {
                        x: x as u32,
                        y: y as u32,
                    },
                    c,
                )
            })
        })
        .collect();

    Grid { locations }
}

fn place_single_anti_node(
    grid: &Grid<char>,
    left: &Location,
    right: &Location,
) -> Option<Location> {
    let leftx = i32::try_from(left.x).unwrap();
    let lefty = i32::try_from(left.y).unwrap();
    let dx: i32 = leftx - i32::try_from(right.x).unwrap();
    let dy: i32 = lefty - i32::try_from(right.y).unwrap();
    if let (Ok(x), Ok(y)) = (u32::try_from(leftx + dx), u32::try_from(lefty + dy)) {
        let placement = Location { x, y };
        if let Some(c) = grid.get_by_location(&placement) {
            return Some(placement);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    let antennas = &grid
        .locations
        .clone()
        .into_iter()
        .filter(|(_, c)| *c != '.')
        .into_group_map_by(|(_, c)| *c)
        .into_iter()
        .collect_vec();

    let anti_nodes: HashSet<Location> = antennas
        .iter()
        .flat_map(|(_, group)| {
            let combinations = group
                .iter()
                .map(|(location, c)| location)
                .combinations(2)
                .collect_vec();

            let t = combinations
                .iter()
                .flat_map(|locations| {
                    let (left, right) = locations.iter().collect_tuple().unwrap();
                    let left_anti_node = place_single_anti_node(&grid, left, right);
                    let right_anti_node = place_single_anti_node(&grid, right, left);
                    vec![left_anti_node, right_anti_node]
                })
                .flatten()
                .collect_vec();
            t
        })
        .collect();

    grid.display(Some(&anti_nodes));
    Some(anti_nodes.len().try_into().unwrap())
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
