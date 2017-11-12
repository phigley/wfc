extern crate rand;
extern crate wfc;

use wfc::field::Field;
use wfc::entry::CharacterEntry;
use wfc::entry;
use rand::Rng;

fn main() {
    //let characters = ['─', '┌', '┐', '│', '└', '┘', ' '];

    if let Err(msg) = execute() {
        println!("{}", msg);
    }
}

fn execute() -> Result<(), String> {
    let potentials = [
        CharacterEntry::build('─', 10.0, "000|101|000")?,
        CharacterEntry::build('│', 10.0, "010|000|010")?,
        CharacterEntry::build('┌', 1.0, "000|001|010")?,
        CharacterEntry::build('┐', 1.0, "000|100|010")?,
        CharacterEntry::build('└', 1.0, "010|001|000")?,
        CharacterEntry::build('┘', 1.0, "010|100|000")?,
        // Notice that adding the ' ' character causes many failures!
        // Not re-assuring for this technique to be used as a decent
        // constraint solution.
        // CharacterEntry::build(' ', 1.0, "000|000|000")?,
    ];

    let mut field = Field::new(&potentials, 80, 40);

    if field.close_edges() {
        let mut rng = rand::thread_rng();

        for i in 0..20 {
            match run_field(&potentials, field.clone(), &mut rng) {
                Ok(result) => {
                    println!("Attemt {} succeeded :", i);
                    println!("{}", result);
                    break;
                }

                Err(result) => {
                    println!("Attempt {} failed:", i);
                    println!("{}", result);
                }
            }
        }
        Ok(())
    } else {
        Err(String::from("Could not close edges"))
    }
}

fn run_field<R: Rng>(
    potentials: &[CharacterEntry],
    mut field: Field,
    mut rng: &mut R,
) -> Result<String, String> {
    loop {
        if let Some(indices) = field.render() {
            let result = entry::make_string(potentials, &indices);

            return Ok(result);
        }

        if field.step(&mut rng) == false {
            let indices = field.render_partial();
            let result = entry::make_string(potentials, &indices);

            return Err(result);
        }
    }
}
