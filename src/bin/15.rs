use advent_of_code::{Direction, Grid, Location};
use itertools::Itertools;
advent_of_code::solution!(15);

fn parse(input: &str) -> Grid<char> {
    Grid::parse(input, |c| match c {
        // robot
        '@' => Some('@'),
        // box
        'O' => Some('O'),
        // wall
        '#' => Some('#'),
        // ground
        '.' => Some('.'),
        _ => None,
    })
}

fn find_boxes(grid: &Grid<char>, first_box: Location, direction: Direction) -> Vec<Location> {
    let mut boxes = vec![first_box];

    loop {
        let Some((new_search_location, search_value)) =
            grid.get_by_direction(boxes.iter().last().unwrap(), direction)
        else {
            break;
        };

        if *search_value != 'O' {
            break;
        }

        boxes.push(new_search_location);
    }

    boxes
}

fn step(grid: &mut Grid<char>, current_location: &mut Location, direction: Direction) {
    // outside
    let Some((new_location, value)) = grid.get_by_direction(current_location, direction) else {
        return;
    };

    match value {
        // box
        'O' => {
            // find boxes ahead
            let boxes = find_boxes(grid, new_location, direction);
            if boxes.is_empty() {
                return;
            }

            let first_box = boxes.first().unwrap();
            let last_box = boxes.last().unwrap();
            let next_location = grid.get_by_direction(last_box, direction);

            let Some(next_location) = next_location else {
                return;
            };

            if *next_location.1 != '.' {
                return;
            }

            grid.locations.insert(next_location.0, 'O');
            grid.locations.insert(*first_box, '.');
        }
        // wall
        '#' => return,
        // ground
        '.' => {}
        _ => panic!(),
    };

    *current_location = new_location;
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut grid = parse(grid);

    let mut current_location = *grid.locations.iter().find(|(_, c)| **c == '@').unwrap().0;
    grid.locations.insert(current_location, '.');

    let moves = moves
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '^' => Direction::Up,
            _ => todo!("unknown character '{c}'"),
        })
        .collect_vec();

    // initial location
    grid.display_location(&current_location);

    for (i, direction) in moves.iter().enumerate() {
        // println!();
        // println!("====================");
        // println!("step: {i}, going: {:?}", direction);
        // grid.display_location(&current_location);
        // let old_grid = grid.clone();
        step(&mut grid, &mut current_location, *direction);

        // if old_grid != grid {
        // grid.display_location(&current_location);
        // }
    }
    grid.display_location(&current_location);

    Some(
        grid.locations
            .iter()
            .filter(|(_, c)| **c == 'O')
            .map(|(l, _)| l.x + l.y * 100)
            .sum(),
    )
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_small() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_single_box() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(104));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
