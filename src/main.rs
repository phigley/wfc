extern crate rand;
extern crate wfc;

use wfc::field::Field;
use wfc::entry::Entry;
use wfc::entry;
use rand::Rng;

fn main() {
    //let characters = ['─', '┌', '┐', '│', '└', '┘', ' '];

    let potentials = [
        Entry::new('─', 10.0, false, false, true, true),
        Entry::new('│', 10.0, true, true, false, false),
        Entry::new('┌', 1.0, false, true, true, false),
        Entry::new('┐', 1.0, false, true, false, true),
        Entry::new('└', 1.0, true, false, true, false),
        Entry::new('┘', 1.0, true, false, false, true),
        // Notice that adding the ' ' character causes many failures!
        // Not re-assuring for this technique to be used as a decent
        // constraint solution.
        // Entry::new(' ', 1.0, false, false, false, false),
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
    } else {
        println!("Could not close edges");
    }
}

fn run_field<R: Rng>(
    potentials: &[Entry],
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
