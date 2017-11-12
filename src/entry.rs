
use boundary::Boundary;

pub trait Entry {
    fn weight(&self) -> f32;
    fn boundary(&self) -> &Boundary;
}

pub struct CharacterEntry {
    pub character: char,
    pub weight: f32,
    boundary: Boundary,
}

impl CharacterEntry {
    pub fn build(character: char, weight: f32, borders: &str) -> Result<CharacterEntry, String> {
        let boundary = Boundary::from_str(borders)?;

        Ok(CharacterEntry {
            character,
            weight,
            boundary,
        })
    }
}

impl Entry for CharacterEntry {
    fn weight(&self) -> f32 {
        self.weight
    }

    fn boundary(&self) -> &Boundary {
        &self.boundary
    }
}

pub fn make_string(potentials: &[CharacterEntry], indices: &[Vec<usize>]) -> String {
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
