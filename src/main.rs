
extern crate rand;

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
        Entry::new('─', false, false, true, true),
        Entry::new('┌', false, true, true, false),
        Entry::new('┐', false, true, false, true),
        Entry::new('│', true, true, false, false),
        Entry::new('└', true, false, true, false),
        Entry::new('┘', true, false, false, true),
        Entry::new(' ', false, false, false, false),
    ];

    let mut field = Field::new(&potentials, 80, 40);

    if field.close_edges() {
        let mut rng = rand::thread_rng();

        loop {

            if let Some(result) = field.render() {
                println!("{}", result);
                break;
            }

            if field.step(&mut rng) == false {
                println!("Failed.");
                break;
            }

        }

    } else {
        println!("Could not close edges");
    }

}
