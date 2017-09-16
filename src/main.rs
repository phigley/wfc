
mod changequeue;
mod boundary;
mod containerutils;
mod entry;
mod field;

use field::Field;
use entry::Entry;

fn main() {

    // let characters = ['─', '┌', '┐', '│', '└', '┘', ' '];

    // let potentials = [
    //     Entry::new('─', false, false, true, true),
    //     Entry::new('┌', false, true, true, false),
    //     Entry::new('┐', false, true, false, true),
    //     Entry::new('│', true, true, false, false),
    //     Entry::new('└', true, false, true, false),
    //     Entry::new('┘', true, false, false, true),
    //     Entry::new(' ', false, false, false, false),
    // ];

    // let mut field = Field::new(&potentials, 4, 3);

    // let mut test = String::new();

    // test.push(potentials[1].character);
    // test.push(potentials[0].character);
    // test.push(potentials[0].character);
    // test.push(potentials[2].character);
    // test.push('\n');

    // test.push(potentials[3].character);
    // test.push(potentials[6].character);
    // test.push(potentials[6].character);
    // test.push(potentials[3].character);
    // test.push('\n');

    // test.push(potentials[4].character);
    // test.push(potentials[0].character);
    // test.push(potentials[0].character);
    // test.push(potentials[5].character);
    // test.push('\n');

    // println!("{}", test);

    Field::simple_test();
}
