use std::string::String;
use std::str;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_index(&self) -> usize {
        match *self {
            Direction::North => 1,
            Direction::East => 3,
            Direction::South => 5,
            Direction::West => 7,
        }
    }

    fn to_opposite_index(&self) -> usize {
        match *self {
            Direction::North => 5,
            Direction::East => 7,
            Direction::South => 1,
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
                None,
                Some(Direction::North),
                None,
                None, // dividing character
                Some(Direction::West),
                None, // center character
                Some(Direction::East),
                None, // dividing character
                None,
                Some(Direction::South),
                None,
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

        assert!(n.fits(&s, Direction::North));
        assert!(!n.fits(&n, Direction::South));

        assert!(n_e.fits(&e_w, Direction::East));
        assert!(!n_e.fits(&e_w, Direction::North));

        assert!(e_w.fits(&e_w, Direction::East));
        assert!(e_w.fits(&n_e, Direction::West));

        assert!(n_e.fits(&s_e, Direction::North));
        assert!(!n_e.fits(&s_e, Direction::East));
    }
}
