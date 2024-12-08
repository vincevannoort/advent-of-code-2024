use advent_of_code::{Grid, Location};
use indoc::indoc;
use itertools::{any, Itertools};
use std::collections::HashMap;

advent_of_code::solution!(4);

fn parse(input: &str) -> Grid<char> {
    // let locations: HashMap<Location, char> = input
    let locations: HashMap<Location, char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, char)| {
                (
                    Location {
                        x: x as u32,
                        y: y as u32,
                    },
                    char,
                )
            })
        })
        .collect();

    Grid { locations }
}

fn is_word((a, b, c, d): &(&char, &char, &char, &char)) -> bool {
    **a == 'X' && **b == 'M' && **c == 'A' && **d == 'S'
}

fn count_words(chars: &Vec<&char>) -> usize {
    let counts_forward = chars
        .clone()
        .into_iter()
        .tuple_windows::<(_, _, _, _)>()
        .filter(is_word)
        .count();
    let counts_backward = chars
        .clone()
        .into_iter()
        .rev()
        .tuple_windows::<(_, _, _, _)>()
        .filter(is_word)
        .count();
    counts_forward + counts_backward
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    grid.display(None);

    let max_location = grid.max_location();

    let mut count = 0;

    // horizontal
    for y in 0..=max_location.y {
        let horizontal_row: Vec<&char> = (0..=max_location.x)
            .filter_map(|x| grid.get(x, y))
            .collect_vec();

        count += count_words(&horizontal_row);
    }

    // vertical
    for x in 0..=max_location.x {
        let vertical_row = (0..=max_location.y)
            .filter_map(|y| grid.get(x, y))
            .collect_vec();

        count += count_words(&vertical_row);
    }

    // diagonal 1 (left to right up)
    for y in 0..=(max_location.y + max_location.x) {
        let diagonal_row: Vec<&char> = (0..=max_location.x)
            .filter_map(|x| {
                if x > y {
                    return None;
                }
                grid.get(x, y - x)
            })
            .collect_vec();

        count += count_words(&diagonal_row);
    }

    // diagonal 2 (left to right down)
    for x in 0..=max_location.x {
        let diagonal_row: Vec<&char> = (0..=max_location.y)
            .filter_map(|y| grid.get(x + y, y))
            .collect_vec();

        count += count_words(&diagonal_row);
    }
    // TODO: how can we do this better?
    for y in 1..=max_location.y {
        let diagonal_row: Vec<&char> = (0..=max_location.x)
            .filter_map(|x| grid.get(x, y + x))
            .collect_vec();

        count += count_words(&diagonal_row);
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    grid.display(None);

    let patterns = grid
        .locations
        .iter()
        .filter(|(_, c)| **c == 'A')
        .filter_map(|(loc, _)| {
            let top_left = grid.get_by_location(&loc.top_left()?)?;
            let top_right = grid.get_by_location(&loc.top_right()?)?;
            let bottom_left = grid.get_by_location(&loc.bottom_left()?)?;
            let bottom_right = grid.get_by_location(&loc.bottom_right()?)?;

            match (top_left, top_right, bottom_right, bottom_left) {
                // M.M
                // .A.
                // S.S
                ('M', 'M', 'S', 'S') => Some(true),
                // S.M
                // .A.
                // S.M
                ('S', 'M', 'M', 'S') => Some(true),
                // S.S
                // .A.
                // M.M
                ('S', 'S', 'M', 'M') => Some(true),
                // M.S
                // .A.
                // M.S
                ('M', 'S', 'S', 'M') => Some(true),
                _ => Some(false),
            }
        })
        .map(|b| b as u32)
        .sum();

    Some(patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
