#![feature(test)]

extern crate rand;
extern crate test;
extern crate wfc;

use rand::{Isaac64Rng, Rng};

use wfc::field::Field;
use wfc::entry::CharacterEntry;

#[bench]
fn closed_box(b: &mut test::Bencher) {
    let potentials = [
        CharacterEntry::build('─', 10.0, "000|101|000").unwrap(),
        CharacterEntry::build('│', 10.0, "010|000|010").unwrap(),
        CharacterEntry::build('┌', 1.0, "000|001|010").unwrap(),
        CharacterEntry::build('┐', 1.0, "000|100|010").unwrap(),
        CharacterEntry::build('└', 1.0, "010|001|000").unwrap(),
        CharacterEntry::build('┘', 1.0, "010|100|000").unwrap(),
        // Notice that adding the ' ' character causes many failures!
        // Not re-assuring for this technique to be used as a decent
        // constraint solution.
        // CharacterEntry::build(' ', 1.0, "000|000|000").unwrap(),
    ];

    b.iter(|| {
        let mut rng = Isaac64Rng::new_unseeded();

        let mut field = Field::new(&potentials, 60, 30);
        if field.close_edges() {
            for _ in 0..100 {
                if run_field(field.clone(), &mut rng) {
                    break;
                }
            }
        }
    });
}


fn run_field<R: Rng> (mut field: Field, mut rng: &mut R) -> bool {
    loop {
        if let Some(_) = field.render() {
            return true;
        }

        if field.step(&mut rng) == false {
            return false;
        }
    }
}

