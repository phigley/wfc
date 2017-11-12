extern crate rand;
extern crate wfc;

use wfc::entry;
use wfc::entry::CharacterEntry;
use wfc::field::Field;

#[test]
fn simple_field_propagate_fail() {
    let potentials = [
        CharacterEntry::build('-', 1.0, "000|101|000").unwrap(),
        CharacterEntry::build('┌', 1.0, "000|001|010").unwrap(),
    ];

    let mut field = Field::new(&potentials, 2, 2);
    assert_eq!(field.force_potential(0, 0, 1), false);
}

#[test]
fn simple_field_full_propagate() {
    let potentials = [
        CharacterEntry::build('┌', 1.0, "000|001|010").unwrap(),
        CharacterEntry::build('┐', 1.0, "000|100|010").unwrap(),
        CharacterEntry::build('└', 1.0, "010|001|000").unwrap(),
        CharacterEntry::build('┘', 1.0, "010|100|000").unwrap(),
    ];

    let mut field = Field::new(&potentials, 2, 2);

    assert!(field.force_potential(0, 0, 0));
    assert!(field.force_potential(1, 0, 1));
    assert!(field.force_potential(0, 1, 2));

    if let Some(result) = field.render() {
        let expected = "┌┐\n└┘\n";
        let result_str = entry::make_string(&potentials, &result);
        assert_eq!(result_str, expected);
    } else {
        let partial_result = field.render_partial();
        let partial_result_str = entry::make_string(&potentials, &partial_result);
        panic!(
            "Field did not fully propagate. Current status:\n{}",
            partial_result_str
        );
    }
}

#[test]
fn simple_field_closed_edges_fail() {
    let potentials = [
        CharacterEntry::build('-', 1.0, "000|101|000").unwrap(),
        CharacterEntry::build('|', 1.0, "010|000|010").unwrap(),
    ];

    let mut field = Field::new(&potentials, 2, 2);
    assert_eq!(field.close_edges(), false);
}

#[test]
fn simple_field_closed_edges() {
    let potentials = [
        CharacterEntry::build('-', 1.0, "000|101|000").unwrap(),
        CharacterEntry::build('|', 1.0, "010|000|010").unwrap(),
        CharacterEntry::build(' ', 1.0, "000|000|000").unwrap(),
    ];

    let mut field = Field::new(&potentials, 2, 2);

    assert!(field.close_edges());

    if let Some(result) = field.render() {
        let expected = "  \n  \n";
        let result_str = entry::make_string(&potentials, &result);
        assert_eq!(result_str, expected);
    } else {
        panic!("Field did not fully close.");
    }
}

#[test]
fn simple_field_closed_edges2() {
    let potentials = [
        CharacterEntry::build('┌', 1.0, "000|001|010").unwrap(),
        CharacterEntry::build('┐', 1.0, "000|100|010").unwrap(),
        CharacterEntry::build('└', 1.0, "010|001|000").unwrap(),
        CharacterEntry::build('┘', 1.0, "010|100|000").unwrap(),
    ];

    let mut field = Field::new(&potentials, 2, 2);

    assert!(field.close_edges());

    if let Some(result) = field.render() {
        let expected = "┌┐\n└┘\n";
        let result_str = entry::make_string(&potentials, &result);
        assert_eq!(result_str, expected);
    } else {
        panic!("Field did not fully close.");
    }
}
