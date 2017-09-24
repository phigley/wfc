
use boundary::Boundary;

pub struct Entry {
    pub character: char,
    pub(crate) boundary: Boundary,
}

impl Entry {
    pub fn new(character: char, north: bool, south: bool, east: bool, west: bool) -> Entry {
        Entry {
            character,
            boundary: Boundary::new(north, south, east, west),
        }
    }
}

pub fn make_string(potentials: &[Entry], indices: &[Vec<usize>]) -> String {

    let mut result = String::new();

    for row in indices {
        for i in row {
            result.push(potentials[*i].character);
        }

        result.push('\n');
    }

    result
}
