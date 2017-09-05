fn main() {

    let characters = ['─', '┌', '┐', '│', '└', '┘', ' '];

    let mut test = String::new();

    test.push(characters[1]);
    test.push(characters[0]);
    test.push(characters[0]);
    test.push(characters[2]);
    test.push('\n');

    test.push(characters[3]);
    test.push(characters[6]);
    test.push(characters[6]);
    test.push(characters[3]);
    test.push('\n');

    test.push(characters[4]);
    test.push(characters[0]);
    test.push(characters[0]);
    test.push(characters[5]);
    test.push('\n');

    println!("{}", test);
}
