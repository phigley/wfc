
use boundary::Boundary;

pub struct Entry {
    pub character: char,
    pub weight: f32,
    pub(crate) boundary: Boundary,
}

impl Entry {
    pub fn new(
        character: char,
        weight: f32,
        north: bool,
        south: bool,
        east: bool,
        west: bool,
    ) -> Entry {
        Entry {
            character,
            weight,
            boundary: Boundary::new(north, south, east, west),
        }
    }
}

pub fn make_string(potentials: &[Entry], indices: &[Vec<usize>]) -> String {

    let mut result = String::new();

    for row in indices {
        for i in row {
            if *i < potentials.len() {
                result.push(potentials[*i].character);
            } else if *i == potentials.len() {
                result.push('?');
            } else {
                result.push('!');
            }

        }
        result.push('\n');
    }

    result
}
