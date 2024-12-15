use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    hash::Hash,
};

use advent_of_code::{Grid, Location};
use itertools::Itertools;

advent_of_code::solution!(14);

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord)]

struct Robot {
    start: Location,
    velocity: Velocity,
}

#[derive(PartialEq, Clone, Debug, Eq, Hash, PartialOrd, Ord)]
struct Robots(Vec<Robot>);

impl Display for Robots {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.len())?;
        Ok(())
    }
}

fn parse(input: &str) -> Grid<Robots> {
    let robots: Robots = Robots(
        input
            .lines()
            .map(|line| {
                let (position, velocity) = line.split_once(' ').unwrap();
                let position: Location = position
                    .split_once(',')
                    .map(|(left, right)| Location {
                        x: left.replace("p=", "").parse().unwrap(),
                        y: right.parse().unwrap(),
                    })
                    .unwrap();

                let velocity: Velocity = velocity
                    .split_once(',')
                    .map(|(left, right)| Velocity {
                        x: left.replace("v=", "").parse().unwrap(),
                        y: right.parse().unwrap(),
                    })
                    .unwrap();

                Robot {
                    start: position,
                    velocity,
                }
            })
            .collect_vec(),
    );

    let grouped_robots = HashMap::from_iter(
        robots
            .0
            .into_iter()
            .into_group_map_by(|robot| robot.start)
            .into_iter()
            .map(|(location, robots)| (location, Robots(robots))),
    );

    Grid {
        locations: grouped_robots,
    }
}

fn step(grid: Grid<Robots>) -> Grid<Robots> {
    let robots = grid
        .locations
        .clone()
        .into_iter()
        .flat_map(|(location, robots)| robots.0.into_iter().map(move |robot| (location, robot)))
        .collect_vec();

    dbg!(&robots);

    let moved_robots = robots
        .into_iter()
        .map(|(location, robot)| {
            (
                Location {
                    x: (location.x as i32 + robot.velocity.x) as u32,
                    y: (location.y as i32 + robot.velocity.y) as u32,
                },
                robot,
            )
        })
        .collect_vec();

    dbg!(moved_robots);

    todo!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Grid<Robots> = parse(input);
    grid.display(None);
    let grid = step(grid);
    // grid
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_one_single() {
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
