#![feature(let_chains)]

use std::collections::{HashMap, HashSet};

use advent_of_code::{Grid, Location};
use itertools::Itertools;
use pathfinding::prelude::{astar, astar_bag, astar_bag_collect, dijkstra_all};

advent_of_code::solution!(20);

fn is_wall(grid: &Grid<char>, x: u32, y: u32) -> bool {
    let Some(item) = grid.get(x, y) else {
        return false;
    };

    *item == '#'
}

fn find_cheatable_locations(grid: &Grid<char>) -> HashMap<Location, Vec<(Location, u32)>> {
    let mut cheatables = vec![];

    let mut add_to_cheatable = |(x, y, u)| {
        cheatables.push((x, y, u));
        cheatables.push((y, x, u));
    };

    for y in 0..=grid.max_location().y {
        for x in 0..=grid.max_location().x {
            let Some('.') = grid.get(x, y) else {
                continue;
            };

            // S#E
            if let Some('.') = grid.get(x + 2, y) {
                if is_wall(grid, x + 1, y) {
                    add_to_cheatable((Location { x, y }, Location { x: x + 2, y }, 2));
                }
            }
            // S#
            // #E
            if let Some('.') = grid.get(x + 1, y + 1) {
                if is_wall(grid, x + 1, y) || is_wall(grid, x, y + 1) {
                    add_to_cheatable((Location { x, y }, Location { x: x + 1, y: y + 1 }, 2));
                }
            }
            // S
            // #
            // E
            if let Some('.') = grid.get(x, y + 2) {
                if is_wall(grid, x, y + 1) {
                    add_to_cheatable((Location { x, y }, Location { x, y: y + 2 }, 2));
                }
            }
            // #E
            // S#
            if y >= 1 {
                if let Some('.') = grid.get(x + 1, y - 1) {
                    if is_wall(grid, x, y - 1) || is_wall(grid, x + 1, y) {
                        add_to_cheatable((Location { x, y }, Location { x: x + 1, y: y - 1 }, 2));
                    }
                }
            }
            // // S##E
            // if let Some('.') = grid.get(x + 3, y) {
            //     if is_wall(grid, x + 1, y) && is_wall(grid, x + 2, y) {
            //         add_to_cheatable((Location { x, y }, Location { x: x + 3, y }, 3));
            //     }
            // }
            // // S##
            // // ##E
            // if let Some('.') = grid.get(x + 2, y + 1) {
            //     if (is_wall(grid, x + 1, y) && is_wall(grid, x + 2, y))
            //         || (is_wall(grid, x + 1, y) && is_wall(grid, x + 1, y + 1))
            //         || (is_wall(grid, x, y + 1) && is_wall(grid, x + 1, y + 1))
            //     {
            //         add_to_cheatable((Location { x, y }, Location { x: x + 2, y: y + 1 }, 3));
            //     }
            // }
            // // S#
            // // ##
            // // #E
            // if let Some('.') = grid.get(x + 1, y + 2) {
            //     if (is_wall(grid, x + 1, y) && is_wall(grid, x + 1, y + 1))
            //         || (is_wall(grid, x, y + 1) && is_wall(grid, x + 1, y + 1))
            //         || (is_wall(grid, x, y + 1) && is_wall(grid, x, y + 2))
            //     {
            //         add_to_cheatable((Location { x, y }, Location { x: x + 1, y: y + 2 }, 3));
            //     }
            // }
            // // #.
            // // ##
            // // S#
            // if y >= 2 {
            //     if let Some('.') = grid.get(x + 1, y - 2) {
            //         if (is_wall(grid, x + 1, y) && is_wall(grid, x + 1, y - 1))
            //             || (is_wall(grid, x, y - 1) && is_wall(grid, x + 1, y - 1))
            //             || (is_wall(grid, x, y - 1) && is_wall(grid, x, y - 2))
            //         {
            //             add_to_cheatable((Location { x, y }, Location { x: x + 1, y: y - 2 }, 3));
            //         }
            //     }
            // }
            // // ###
            // // ##.
            // // S##
            // if y >= 1 {
            //     if let Some('.') = grid.get(x + 2, y - 1) {
            //         if (is_wall(grid, x + 1, y) && is_wall(grid, x + 2, y))
            //             || (is_wall(grid, x + 1, y) && is_wall(grid, x + 1, y - 1))
            //             || (is_wall(grid, x, y - 1) && is_wall(grid, x + 1, y - 1))
            //         {
            //             add_to_cheatable((Location { x, y }, Location { x: x + 2, y: y - 1 }, 3));
            //         }
            //     }
            // }
        }
    }

    let cheatables: HashMap<Location, Vec<(Location, u32)>> =
        cheatables
            .into_iter()
            .fold(HashMap::new(), |mut acc, (x, y, u)| {
                acc.entry(x)
                    .and_modify(|t| t.push((y, u)))
                    .or_insert(vec![(y, u)]);
                acc
            });

    cheatables
}

#[derive(PartialEq, Clone, Debug, Eq, Hash, PartialOrd, Ord)]
struct Cheated(bool);

fn get_successors(grid: &Grid<char>, p: &Location) -> Vec<(Location, u32)> {
    grid.get_surrounding_locations(p)
        .iter()
        .filter(|l| *l.1 == '.')
        .map(|l| (l.0, 1))
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::parse(input, Some);
    grid.display(None);
    let (start, _) = grid.find_point_of_interest_and_replace('S', '.');
    let (end, _) = grid.find_point_of_interest_and_replace('E', '.');
    let cheatable_locations = find_cheatable_locations(&grid);

    let mut start_to_any = dijkstra_all(&(start), |p| get_successors(&grid, p));
    let mut end_to_any = dijkstra_all(&(end), |p| get_successors(&grid, p));
    start_to_any.insert(start, (start, 0));
    end_to_any.insert(end, (end, 0));
    let normal_length = start_to_any.get(&end).unwrap().1;

    let cheatable_locations_best_paths = cheatable_locations
        .iter()
        .flat_map(|(start, ends)| ends.iter().map(|end| (start.clone(), end.0, end.1)))
        .filter_map(|(cheat_start, cheat_end, cost)| {
            let from_start_to_cheat_start = start_to_any.get(&cheat_start).unwrap();
            let from_finish_to_cheat_end = end_to_any.get(&cheat_end).unwrap();

            let cost = from_start_to_cheat_start.1 + cost + from_finish_to_cheat_end.1;
            if cost > normal_length {
                return None;
            }
            Some(normal_length - cost)
        })
        .sorted()
        .collect_vec();

    let count = cheatable_locations_best_paths
        .into_iter()
        .filter(|c| *c >= 100)
        .count();

    Some(count as u32)
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
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
