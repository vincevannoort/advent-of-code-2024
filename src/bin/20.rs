#![feature(let_chains)]

use std::collections::{HashMap, HashSet};

use advent_of_code::{Grid, Location};
use itertools::Itertools;
use pathfinding::prelude::{astar, astar_bag, astar_bag_collect};

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

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::parse(input, Some);
    grid.display(None);
    let (start, _) = grid.find_point_of_interest_and_replace('S', '.');
    let (end, _) = grid.find_point_of_interest_and_replace('E', '.');
    let cheatable_locations = find_cheatable_locations(&grid);

    let normal_path = astar(
        &(start),
        |p| {
            let surrounding_locations = grid
                .get_surrounding_locations(p)
                .iter()
                .filter(|l| *l.1 == '.')
                .map(|l| (l.0, 1))
                .collect_vec();

            surrounding_locations
        },
        // manhattan distance for heuristic
        |p| p.y.abs_diff(end.y) + p.x.abs_diff(end.x),
        |p| *p == end,
    )
    .unwrap();

    let cheatable_locations_best_paths = cheatable_locations
        .iter()
        // .filter(|p| p.0.x == 7 && p.0.y == 7)
        .flat_map(|(start, ends)| ends.iter().map(|end| (start.clone(), end.0, end.1)))
        .filter_map(|(cheat_start, cheat_end, cost)| {
            let path = astar(
                &(start, Cheated(false)),
                |(p, cheated)| {
                    let mut surrounding_locations = grid
                        .get_surrounding_locations(p)
                        .iter()
                        .filter(|l| *l.1 == '.')
                        .map(|l| ((l.0, cheated.clone()), 1))
                        .collect_vec();

                    // if we havent cheated yet
                    if !cheated.0 && *p == cheat_start {
                        surrounding_locations.push(((cheat_end, Cheated(true)), cost));
                    }

                    surrounding_locations
                },
                // manhattan distance for heuristic
                |(p, cheated)| p.y.abs_diff(end.y) + p.x.abs_diff(end.x),
                |(p, cheated)| *p == end,
            )
            .unwrap();

            // if the shortest path does not include the cheat
            if !path.0.last().unwrap().1 .0 {
                return None;
            }

            // if the shortest path that does include cheat is not faster
            if normal_path.1 == path.1 {
                return None;
            }

            Some(normal_path.1 - path.1)
        })
        .sorted()
        .collect_vec();

    // dbg!(&cheatable_locations_best_paths);

    let mut total = 0;
    let mut count = 0;
    for cheat in cheatable_locations_best_paths.iter().rev() {
        total += cheat;
        count += 1;
        if total >= 100 {
            break;
        }
    }

    Some(count)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
