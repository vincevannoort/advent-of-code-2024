use colored::Colorize;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};

pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord)]
pub struct Location {
    pub x: u32,
    pub y: u32,
}

impl Location {
    pub fn top_left(&self) -> Option<Location> {
        if (self.x == 0 || self.y == 0) {
            return None;
        }
        Some(Location {
            x: self.x - 1,
            y: self.y - 1,
        })
    }
    pub fn top_right(&self) -> Option<Location> {
        if (self.y == 0) {
            return None;
        }
        Some(Location {
            x: self.x + 1,
            y: self.y - 1,
        })
    }
    pub fn bottom_left(&self) -> Option<Location> {
        if self.x == 0 {
            return None;
        }
        Some(Location {
            x: self.x - 1,
            y: self.y + 1,
        })
    }
    pub fn bottom_right(&self) -> Option<Location> {
        Some(Location {
            x: self.x + 1,
            y: self.y + 1,
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Grid<T> {
    pub locations: HashMap<Location, T>,
}

impl<T> Grid<T>
where
    T: Display,
{
    pub fn get(&self, x: u32, y: u32) -> Option<&T> {
        self.locations.get(&Location { x, y })
    }

    pub fn get_by_location(&self, loc: &Location) -> Option<&T> {
        self.locations.get(&loc)
    }

    pub fn max_location(&self) -> Location {
        let (max_x_location, _) = self.locations.iter().max_by_key(|l| l.0.x).unwrap();
        let (max_y_location, _) = self.locations.iter().max_by_key(|l| l.0.y).unwrap();
        Location {
            x: max_x_location.x,
            y: max_y_location.y,
        }
    }

    pub fn display(&self, highlights: Option<&HashSet<Location>>) {
        println!();
        let max_location = self.max_location();
        for y in 0..=max_location.y {
            for x in 0..=max_location.x {
                let location = Location { x, y };
                if let Some(entity) = self.locations.get(&location) {
                    match highlights {
                        Some(highlights) if highlights.contains(&location) => {
                            print!("{}", format!("{entity}").on_bright_magenta())
                        }
                        _ => print!("{entity}"),
                    };
                } else {
                    print!(" ")
                }
            }
            println!();
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
