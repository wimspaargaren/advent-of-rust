use std::{fs};

fn main() {
    let contents =
        fs::read_to_string("./src/day7/input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    _ = lines;
}
