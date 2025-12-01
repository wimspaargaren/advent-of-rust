use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./src/day2/input.txt").expect("Should have been able to read the file");

    _ = contents;
}
