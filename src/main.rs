
extern crate rand;

use std::clone::Clone;
use rand::Rng;

mod changequeue;
mod boundary;
mod containerutils;
mod entry;
mod field;



use field::Field;
use entry::Entry;


fn main() {

    //let characters = ['─', '┌', '┐', '│', '└', '┘', ' '];

    let potentials = [
        Entry::new('─', 10.0, false, false, true, true),
        Entry::new('│', 10.0, true, true, false, false),
        Entry::new('┌', 1.0, false, true, true, false),
        Entry::new('┐', 1.0, false, true, false, true),
        Entry::new('└', 1.0, true, false, true, false),
        Entry::new('┘', 1.0, true, false, false, true),
        Entry::new(' ', 0.5, false, false, false, false),
    ];

    let mut field = Field::new(&potentials, 80, 40);

    if field.close_edges() {
        let mut rng = rand::thread_rng();

        for i in 0..20 {
            match run_field(&potentials, field.clone(), &mut rng) {
                Some(result) => {
                    println!("Attemt {} succeeded :", i);
                    println!("{}", result);
                    break;
                }

                None => println!("Attempt {} failed.", i),
            }
        }

    } else {
        println!("Could not close edges");
    }

}

fn run_field<R: Rng>(potentials: &[Entry], mut field: Field, mut rng: &mut R) -> Option<String> {

    loop {

        if let Some(indices) = field.render() {

            let result = entry::make_string(potentials, &indices);

            return Some(result);
        }

        if field.step(&mut rng) == false {
            return None;
        }
    }
}
