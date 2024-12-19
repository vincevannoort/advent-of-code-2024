use std::collections::HashMap;

use advent_of_code::{Grid, Location};
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

advent_of_code::solution!(18);

fn find_solution_for_bytes(
    locations: &Vec<(Location, char)>,
    end: Location,
    bytes: u32,
) -> Option<u32> {
    println!("{}", bytes);

    // grid with only fallen bytes
    let mut locations = HashMap::from_iter(
        locations
            .clone()
            .into_iter()
            .take(bytes.try_into().unwrap()),
    );
    locations.entry(end).or_insert('.');
    let mut grid: Grid<char> = Grid { locations };

    grid.fill_remaining('.');

    // grid.display(None);

    let result: (Vec<Location>, _) = dijkstra(
        &Location { x: 0, y: 0 },
        |p: &Location| {
            let surrounding_locations = grid
                .get_optional_surrounding_locations(p)
                .into_iter()
                .flatten()
                .filter(|loc| *loc.1 == '.')
                .map(|loc| (loc.0, 1))
                .collect_vec();
            surrounding_locations
        },
        |p| *p == end,
    )?;

    Some((result.0.len() - 1).try_into().unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let locations: Vec<(Location, char)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (
                Location {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                },
                '#',
            )
        })
        .collect_vec();

    // grid with all bytes
    let full_grid: Grid<char> = Grid {
        locations: HashMap::from_iter(locations.clone()),
    };

    let max_location = full_grid.max_location();

    find_solution_for_bytes(&locations, max_location, 1024)
}

pub fn part_two(input: &str) -> Option<u32> {
    let locations: Vec<(Location, char)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (
                Location {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                },
                '#',
            )
        })
        .collect_vec();

    // grid with all bytes
    let full_grid: Grid<char> = Grid {
        locations: HashMap::from_iter(locations.clone()),
    };

    let max_location = full_grid.max_location();

    let result = (1..locations.len())
        .find(|bytes| find_solution_for_bytes(&locations, max_location, *bytes as u32).is_none());

    result.map(|result| result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
