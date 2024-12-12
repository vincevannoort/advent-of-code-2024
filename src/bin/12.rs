use std::collections::{HashMap, HashSet};

use advent_of_code::{Grid, Location};
use itertools::Itertools;

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

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input, Some);

    let plots: HashMap<Location, usize> = grid
        .locations
        .iter()
        .sorted()
        .tuple_windows()
        .filter(|(left, right)| left.1 != right.1)
        .fold(HashMap::new(), |mut acc, (left, _)| {
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
    let plots: Vec<HashSet<Location>> = plots
        .iter()
        .into_group_map_by(|plot| plot.1)
        .into_iter()
        .map(|plot| HashSet::from_iter(plot.1.iter().map(|plant| plant.0.clone())))
        .collect_vec();

    let result = plots.iter().map(|plot| calculate_price(&grid, plot)).sum();

    Some(result)
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
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
