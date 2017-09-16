#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Boundary {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl Boundary {
    pub fn new(north: bool, south: bool, east: bool, west: bool) -> Boundary {
        Boundary {
            north,
            east,
            south,
            west,
        }
    }

    // returns true if the other fits on direction side.
    pub fn fits(&self, other: &Boundary, direction: Direction) -> bool {
        match direction {
            Direction::North => self.north == other.south,
            Direction::East => self.east == other.west,
            Direction::South => self.south == other.north,
            Direction::West => self.west == other.east,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn boundaries_match() {
        let n = Boundary::new(true, false, false, false);
        let s = Boundary::new(false, true, false, false);

        let ne = Boundary::new(true, false, true, false);
        let se = Boundary::new(false, true, true, false);

        let ew = Boundary::new(false, false, true, true);

        assert!(n.fits(&s, Direction::North));
        assert!(!n.fits(&n, Direction::South));

        assert!(ne.fits(&ew, Direction::East));
        assert!(!ne.fits(&ew, Direction::North));

        assert!(ew.fits(&ew, Direction::East));
        assert!(ew.fits(&ne, Direction::West));

        assert!(ne.fits(&se, Direction::North));
        assert!(!ne.fits(&se, Direction::East));
    }
}
