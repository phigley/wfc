use std::string::String;
use std::str;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

impl Direction {
    pub const ALL_DIRECTIONS: [Direction; 8] = [
        Direction::NorthWest,
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
    ];

    fn to_index(&self) -> usize {
        match *self {
            Direction::NorthWest => 0,
            Direction::North => 1,
            Direction::NorthEast => 2,
            Direction::East => 3,
            Direction::SouthEast => 4,
            Direction::South => 5,
            Direction::SouthWest => 6,
            Direction::West => 7,
        }
    }

    fn to_opposite_index(&self) -> usize {
        match *self {
            Direction::NorthWest => 4,
            Direction::North => 5,
            Direction::NorthEast => 6,
            Direction::East => 7,
            Direction::SouthEast => 0,
            Direction::South => 1,
            Direction::SouthWest => 2,
            Direction::West => 3,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Boundary {
    boundaries: [bool; 8],
}

impl Boundary {
    pub fn from_str(borders: &str) -> Result<Boundary, String> {
        if borders.len() != 11 {
            Err(format!(
                "Boundary::from_str(\"{}\") is incorrect.  Input length is {}, expected 11.",
                borders,
                borders.len()
            ))
        } else {
            let directions = [
                Some(Direction::NorthWest),
                Some(Direction::North),
                Some(Direction::NorthEast),
                None, // dividing character
                Some(Direction::West),
                None, // center character
                Some(Direction::East),
                None, // dividing character
                Some(Direction::SouthWest),
                Some(Direction::South),
                Some(Direction::SouthEast),
            ];

            let mut result = Boundary::default();

            for (c, possible_direction) in borders.as_bytes().iter().zip(&directions) {
                if let Some(direction) = *possible_direction {
                    if *c == ('1' as u8) {
                        result.boundaries[direction.to_index()] = true;
                    } else if *c != ('0' as u8) {
                        return Err(format!(
                            "Found invalid character '{}' in Bound::from_str(\"{}\")",
                            (*c as char),
                            borders
                        ));
                    }
                }
            }

            Ok(result)
        }
    }

    // returns true if the other fits on direction side.
    pub fn fits(&self, other: &Boundary, direction: Direction) -> bool {
        self.boundaries[direction.to_index()] == other.boundaries[direction.to_opposite_index()]
    }

    pub fn requires(&self, direction: Direction) -> bool {
        self.boundaries[direction.to_index()]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn boundaries_match() {
        let n = Boundary::from_str("010|000|000").unwrap();
        let s = Boundary::from_str("000|000|010").unwrap();
        let n_e = Boundary::from_str("010|001|000").unwrap();
        let s_e = Boundary::from_str("000|001|010").unwrap();
        let e_w = Boundary::from_str("000|101|000").unwrap();

        let ne = Boundary::from_str("001|000|000").unwrap();
        let sw = Boundary::from_str("000|000|100").unwrap();

        let nw_e = Boundary::from_str("100|001|000").unwrap();
        let se_e_w = Boundary::from_str("000|101|001").unwrap();

        assert!(n.fits(&s, Direction::North));
        assert!(!n.fits(&n, Direction::South));

        assert!(n_e.fits(&e_w, Direction::East));
        assert!(!n_e.fits(&e_w, Direction::North));

        assert!(e_w.fits(&e_w, Direction::East));
        assert!(e_w.fits(&n_e, Direction::West));

        assert!(n_e.fits(&s_e, Direction::North));
        assert!(!n_e.fits(&s_e, Direction::East));

        assert!(ne.fits(&sw, Direction::NorthEast));
        assert!(!ne.fits(&nw_e, Direction::NorthEast));

        assert!(nw_e.fits(&e_w, Direction::East));
        assert!(e_w.fits(&nw_e, Direction::West));

        assert!(se_e_w.fits(&nw_e, Direction::SouthEast));
    }
}
