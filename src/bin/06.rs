use advent_of_code::{Direction, Grid, Location};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

advent_of_code::solution!(6);

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord)]
enum Entity {
    Ground,
    Obstacle,
    Guard(Direction),
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Obstacle => write!(f, "#"),
            Entity::Guard(_) => write!(f, "G"),
            Entity::Ground => write!(f, "."),
        }
    }
}

fn parse(input: &str) -> Grid<Entity> {
    let obstacles: HashMap<Location, Entity> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().flat_map(move |(x, char)| {
                let location = Location {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };
                match char {
                    '.' => Some((location, Entity::Ground)),
                    '#' => Some((location, Entity::Obstacle)),
                    '^' => Some((location, Entity::Guard(Direction::Up))),
                    _ => panic!("unknown char: {}", char),
                }
            })
        })
        .collect();

    Grid {
        locations: obstacles,
    }
}

fn update_location(guard_location: Location, guard_direction: Direction) -> Location {
    match guard_direction {
        Direction::Up => Location {
            x: guard_location.x,
            y: guard_location.y - 1,
        },
        Direction::Right => Location {
            x: guard_location.x + 1,
            y: guard_location.y,
        },
        Direction::Down => Location {
            x: guard_location.x,
            y: guard_location.y + 1,
        },
        Direction::Left => Location {
            x: guard_location.x - 1,
            y: guard_location.y,
        },
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);

    let (mut guard_location, mut guard_direction) = grid
        .locations
        .iter()
        .find(|(_, e)| matches!(e, Entity::Guard(_)))
        .map(|e| (*e.0, Direction::Up))
        .unwrap();

    let mut visited_locations: HashSet<Location> = HashSet::new();
    visited_locations.insert(guard_location);
    loop {
        if (visited_locations.len() % 200 == 0) {
            grid.display(Some(&visited_locations));
        }

        let new_guard_location = update_location(guard_location, guard_direction);
        let entity = grid.get_by_location(&new_guard_location);

        let Some(entity) = entity else {
            break;
        };

        // found obstace, turn right
        if let Some(new_direction) = match (entity, guard_direction) {
            (Entity::Obstacle, Direction::Up) => Some(Direction::Right),
            (Entity::Obstacle, Direction::Right) => Some(Direction::Down),
            (Entity::Obstacle, Direction::Down) => Some(Direction::Left),
            (Entity::Obstacle, Direction::Left) => Some(Direction::Up),
            _ => None,
        } {
            guard_direction = new_direction;
        }

        // found obstace, turn right
        if matches!(entity, Entity::Ground | Entity::Guard(_)) {
            guard_location = new_guard_location;
        }

        visited_locations.insert(guard_location);
    }

    Some(visited_locations.len() as u32)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
