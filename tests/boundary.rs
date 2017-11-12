extern crate wfc;

use wfc::boundary::{Boundary, Direction};

#[test]
fn build_from_str_north() {
    let source = "010|000|000";

    let boundary = Boundary::from_str(source).unwrap();
    assert!(boundary.requires(Direction::North));
    assert!(!boundary.requires(Direction::South));
    assert!(!boundary.requires(Direction::East));
    assert!(!boundary.requires(Direction::West));
}

#[test]
fn build_from_str_south() {
    let source = "000|000|010";

    let boundary = Boundary::from_str(source).unwrap();
    assert!(!boundary.requires(Direction::North));
    assert!(boundary.requires(Direction::South));
    assert!(!boundary.requires(Direction::East));
    assert!(!boundary.requires(Direction::West));
}

#[test]
fn build_from_str_east() {
    let source = "000|001|000";

    let boundary = Boundary::from_str(source).unwrap();
    assert!(!boundary.requires(Direction::North));
    assert!(!boundary.requires(Direction::South));
    assert!(boundary.requires(Direction::East));
    assert!(!boundary.requires(Direction::West));
}

#[test]
fn build_from_str_west() {
    let source = "000|100|000";

    let boundary = Boundary::from_str(source).unwrap();
    assert!(!boundary.requires(Direction::North));
    assert!(!boundary.requires(Direction::South));
    assert!(!boundary.requires(Direction::East));
    assert!(boundary.requires(Direction::West));
}

#[test]
fn build_from_str_multi() {
    let source = "010|100|010";

    let boundary = Boundary::from_str(source).unwrap();
    assert!(boundary.requires(Direction::North));
    assert!(boundary.requires(Direction::South));
    assert!(!boundary.requires(Direction::East));
    assert!(boundary.requires(Direction::West));
}

#[test]
#[should_panic(expected = "10")]
fn build_from_str_fails_short() {
    let source = "010|00|000";

    let boundary = Boundary::from_str(source).unwrap();
    println!("Built boundary:\n{:?}", boundary);
}

#[test]
#[should_panic(expected = "12")]
fn build_from_str_fails_long() {
    let source = "0100|000|000";

    let boundary = Boundary::from_str(source).unwrap();
    println!("Built boundary:\n{:?}", boundary);
}

#[test]
#[should_panic(expected = "\\'b\\'")]
fn build_from_str_fails_unknown_char() {
    let source = "010|00b|000";

    let boundary = Boundary::from_str(source).unwrap();
    println!("Built boundary:\n{:?}", boundary);
}

#[test]
fn build_succeeds_different_boundary() {
    let source = "010?001%010";

    let boundary = Boundary::from_str(source).unwrap();
    assert!(boundary.requires(Direction::North));
    assert!(boundary.requires(Direction::East));
    assert!(boundary.requires(Direction::South));
}

#[test]
fn build_succeeds_different_center() {
    let source = "010|0*1|010";

    let boundary = Boundary::from_str(source).unwrap();
    assert!(boundary.requires(Direction::North));
    assert!(boundary.requires(Direction::East));
    assert!(boundary.requires(Direction::South));
}
