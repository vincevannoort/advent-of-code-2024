use std::collections::HashSet;

use advent_of_code::{Grid, Location};
use itertools::Itertools;

advent_of_code::solution!(10);

fn parse(input: &str) -> Grid<u32> {
    Grid::parse(input, |c| match c {
        '.' => None,
        _ => Some(c.to_string().parse().unwrap()),
    })
}

fn find_possible_next_paths<'a>(
    grid: &'a Grid<u32>,
    location: &'a Location,
    current_height: &'a u32,
) -> Vec<(Location, &'a u32)> {
    grid.get_surrounding_locations(location)
        .into_iter()
        .filter(|(_, next_height)| {
            let is_higher = *next_height > current_height;
            if !is_higher {
                return false;
            }

            *next_height - *current_height == 1
        })
        .collect_vec()
}

fn traverse_path_part_1(
    grid: &Grid<u32>,
    visited_locations: Vec<Location>,
    location: &Location,
    height: &u32,
) -> HashSet<Location> {
    if *height == 9 {
        // create set with location of 9 to count once
        return HashSet::from_iter(vec![location].into_iter().cloned());
    }

    let surrounding_locations = find_possible_next_paths(grid, location, height);

    // sum valid locations
    surrounding_locations
        .into_iter()
        .flat_map(|(surround_location, new_height)| {
            traverse_path_part_1(
                grid,
                [visited_locations.clone(), vec![surround_location]].concat(),
                &surround_location,
                new_height,
            )
        })
        .collect()
}

fn traverse_path_part_2(
    grid: &Grid<u32>,
    visited_locations: Vec<Location>,
    location: &Location,
    height: &u32,
) -> u32 {
    if *height == 9 {
        // count each valid path
        return 1;
    }

    let surrounding_locations = find_possible_next_paths(grid, location, height);

    // sum valid locations
    surrounding_locations
        .into_iter()
        .map(|(surround_location, new_height)| {
            traverse_path_part_2(
                grid,
                [visited_locations.clone(), vec![surround_location]].concat(),
                &surround_location,
                new_height,
            )
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);

    let zeros = grid
        .locations
        .iter()
        .filter(|(_, height)| **height == 0)
        .collect_vec();

    let result = zeros
        .iter()
        .flat_map(|zero| {
            let valid_paths = traverse_path_part_1(&grid, vec![*zero.0], zero.0, zero.1);
            Some(valid_paths.len() as u32)
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);

    let zeros = grid
        .locations
        .iter()
        .filter(|(_, height)| **height == 0)
        .collect_vec();

    let result = zeros
        .iter()
        .map(|zero| traverse_path_part_2(&grid, vec![*zero.0], zero.0, zero.1))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_one_medium() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_one_dots() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two_dots() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(13));
    }
}
