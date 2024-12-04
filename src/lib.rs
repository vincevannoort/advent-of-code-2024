use std::collections::HashMap;
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
    T: Debug + Display,
{
    pub fn get(&self, x: u32, y: u32) -> Option<&T> {
        self.locations.get(&Location { x, y })
    }

    pub fn get_by_location(&self, loc: &Location) -> Option<&T> {
        self.locations.get(&loc)
    }

    pub fn max_location(&self) -> &Location {
        let (max_location, _) = self.locations.iter().max_by_key(|x| x.0).unwrap();
        max_location
    }

    pub fn display(&self) {
        let max_location = self.max_location();
        for y in 0..=max_location.y {
            for x in 0..=max_location.x {
                let location = self.locations.get(&Location { x, y }).unwrap();
                print!("{location}");
            }
            println!();
        }
    }
}
