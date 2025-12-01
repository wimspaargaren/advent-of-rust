use std::fs;

#[derive(Debug)]
struct Rotation {
    direction: char,
    degrees: u32,
}

fn main() {
    let contents =
        fs::read_to_string("./src/day1/input.txt").expect("Should have been able to read the file");

    let mut rotations = Vec::new();
    // split content on newline
    for line in contents.lines() {
        rotations.push(parse_rotation(line));
    }

    let mut start = 50;
    let mut exact_zero = 0;
    let mut total_zero = 0;

    for rotation in rotations {
        if rotation.direction == 'R' {
            for _ in 0..rotation.degrees {
                start += 1;

                if start == 100 {
                    total_zero += 1;
                    start = 0;
                }
            }
        } else {
            for _ in 0..rotation.degrees {
                start -= 1;
                if start == 0 {
                    total_zero += 1;
                }
                if start == -1 {
                    start = 99;
                }
            }
        }
        if start == 0 {
            exact_zero += 1;
        }
    }
    println!("part 1: {}", exact_zero);
    println!("part 2: {}", total_zero);
}

fn parse_rotation(line: &str) -> Rotation {
    let direction = line.chars().next().unwrap();
    let degrees: u32 = line[1..].parse().unwrap();
    Rotation { direction, degrees }
}
