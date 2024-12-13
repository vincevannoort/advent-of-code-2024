use core::panic;
use std::{
    collections::{HashMap, HashSet},
    vec,
};

use advent_of_code::{Direction, Grid, Location};
use itertools::{min, Itertools};

advent_of_code::solution!(12);

fn fill_surroundings(
    grid: &Grid<char>,
    plots: &mut HashMap<Location, usize>,
    plot: usize,
    location: &(Location, &char),
) {
    // it could be filled in the mean time
    if plots.contains_key(&location.0) {
        return;
    }

    plots.insert(location.0, plot);

    let surroundings = grid.get_surrounding_locations(&location.0);
    let surroundings = surroundings
        .iter()
        .filter(|neighbor| location.1 == neighbor.1)
        .collect_vec();

    if surroundings.is_empty() {
        return;
    }

    for surrounding in surroundings {
        fill_surroundings(grid, plots, plot, surrounding);
    }
}

fn calculate_price(grid: &Grid<char>, plot: &HashSet<Location>) -> u32 {
    let area = plot.len();
    let perimeter: usize = plot
        .iter()
        .map(|plant| {
            let plant_surroundings = grid.get_surrounding_locations(plant);
            let same_plant_surroundings = 4 - plant_surroundings
                .iter()
                .filter(|surrounding| plot.contains(&surrounding.0))
                .count();
            same_plant_surroundings
        })
        .sum();

    (area * perimeter).try_into().unwrap()
}

fn calculate_plots(grid: &Grid<char>) -> Vec<HashSet<Location>> {
    let plots: HashMap<Location, usize> = grid
        .locations
        .iter()
        .sorted()
        .tuple_windows()
        .filter(|(left, right)| left.1 != right.1)
        .fold(HashMap::new(), |mut acc, (left, _)| {
            // is debug group
            if *left.1 == '.' {
                return acc;
            }

            // we already seen this one
            if acc.contains_key(left.0) {
                return acc;
            }

            let unique_ids = acc.values().unique().count();

            // fill surroundings
            fill_surroundings(&grid, &mut acc, unique_ids, &(*left.0, left.1));

            acc
        });

    // group by id
    plots
        .iter()
        .into_group_map_by(|plot| plot.1)
        .into_iter()
        .map(|plot| HashSet::from_iter(plot.1.iter().map(|plant| plant.0.clone())))
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input, Some);
    let plots = calculate_plots(&grid);
    let result = plots.iter().map(|plot| calculate_price(&grid, plot)).sum();

    Some(result)
}

fn calculate_price_with_discount(plot: &HashSet<Location>) -> u32 {
    let area = plot.len();
    const PLOT_ID: char = 'Z';

    let custom_grid = Grid {
        locations: HashMap::from_iter(plot.clone().into_iter().map(|plant| {
            (
                Location {
                    // allow for some space around
                    x: plant.x + 1,
                    y: plant.y + 1,
                },
                PLOT_ID,
            )
        })),
    };

    let top_left = Location {
        x: custom_grid.min_location().x - 1,
        y: custom_grid.min_location().y - 1,
    };

    let bottom_right = Location {
        x: custom_grid.max_location().x + 1,
        y: custom_grid.max_location().y + 1,
    };

    let mut corners = 0;

    for y in top_left.y..bottom_right.y {
        for x in top_left.x..bottom_right.x {
            let location = Location { x, y };
            let top_left = custom_grid.get_by_location(&location);
            let top_right = custom_grid.get_by_location(&Location { x: x + 1, y });
            let bottom_left = custom_grid.get_by_location(&Location { x, y: y + 1 });
            let bottom_right = custom_grid.get_by_location(&Location { x: x + 1, y: y + 1 });

            corners += match (top_left, top_right, bottom_left, bottom_right) {
                (Some('Z'), None, None, None) => 1,
                (None, Some('Z'), None, None) => 1,
                (None, None, Some('Z'), None) => 1,
                (None, None, None, Some('Z')) => 1,
                (Some('Z'), None, None, Some('Z')) => 2,
                (None, Some('Z'), Some('Z'), None) => 2,
                (Some('Z'), Some('Z'), Some('Z'), None) => 1,
                (None, Some('Z'), Some('Z'), Some('Z')) => 1,
                (Some('Z'), None, Some('Z'), Some('Z')) => 1,
                (Some('Z'), Some('Z'), None, Some('Z')) => 1,
                _ => 0,
            };
        }
    }

    (area * corners).try_into().unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::parse(input, Some);

    let plots = calculate_plots(&grid);
    let result = plots.iter().map(calculate_price_with_discount).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_simple() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_hard() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(368));
    }
}
