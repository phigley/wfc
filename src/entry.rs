
use boundary::{Boundary, Direction};

pub struct Entry {
    pub character: char,
    boundary: Boundary,
}

impl Entry {
    pub fn new(character: char, north: bool, south: bool, east: bool, west: bool) -> Entry {
        Entry {
            character,
            boundary: Boundary::new(north, south, east, west),
        }
    }

    pub fn fits(&self, other: &Entry, direction: Direction) -> bool {
        return self.boundary.fits(&other.boundary, direction);
    }
}
