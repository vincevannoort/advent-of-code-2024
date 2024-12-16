use std::collections::HashSet;

use advent_of_code::{Direction, Grid, Location};
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

advent_of_code::solution!(16);

fn remove_entry(grid: &mut Grid<char>, _char: char) -> (Location, char) {
    let removed = grid
        .locations
        .remove_entry(
            &grid
                .locations
                .iter()
                .find(|(_, c)| **c == _char)
                .unwrap()
                .0
                .clone(),
        )
        .unwrap();

    grid.locations.insert(removed.0, '.');

    removed
}

fn map_successor(
    successor: Option<(Location, &char)>,
    current_direction: Direction,
    next_direction: Direction,
) -> Option<((Location, Direction), u32)> {
    let successor = successor?;

    match (current_direction, next_direction) {
        (Direction::Up, Direction::Down) => return None,
        (Direction::Right, Direction::Left) => return None,
        (Direction::Down, Direction::Up) => return None,
        (Direction::Left, Direction::Right) => return None,
        _ => {}
    }

    if *successor.1 != '.' {
        return None;
    }

    Some((
        (successor.0, next_direction),
        if current_direction == next_direction {
            1
        } else {
            1001
        },
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::parse(input, Some);
    let (start, _) = remove_entry(&mut grid, 'S');
    let (end, _) = remove_entry(&mut grid, 'E');

    let result = dijkstra(
        &(start, Direction::Up),
        |p| {
            // get surrounding locations
            let (up, right, down, left) = grid
                .get_optional_surrounding_locations(&p.0)
                .into_iter()
                .collect_tuple()
                .unwrap();

            let up = map_successor(up, p.1, Direction::Up);
            let right = map_successor(right, p.1, Direction::Right);
            let down = map_successor(down, p.1, Direction::Down);
            let left = map_successor(left, p.1, Direction::Left);

            let successors = vec![up, right, down, left]
                .into_iter()
                .flatten()
                .collect_vec();

            // println!();
            // println!("=============================================");
            // grid.display_location(&p.0);
            // dbg!((up, right, down, left));
            // dbg!(p, &successors);

            successors
        },
        |p| p.0 == end,
    )
    .unwrap();

    // display way
    grid.display(Some(&HashSet::from_iter(
        result.0.iter().map(|successor| successor.0).collect_vec(),
    )));

    Some(result.1 + 1000)
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
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
