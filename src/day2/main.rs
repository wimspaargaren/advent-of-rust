use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./src/day2/input.txt").expect("Should have been able to read the file");
    let ranges: Vec<&str> = contents
        .strip_suffix("\n")
        .unwrap_or(&contents)
        .split(",")
        .collect();
    let mut part_1_sum = 0;
    let mut part_2_sum = 0;
    for range in ranges {
        let bounds: Vec<&str> = range.split('-').collect();
        let start = bounds[0].parse::<u64>().unwrap();
        let end = bounds[1].parse::<u64>().unwrap();

        for i in start..=end {
            if is_palindrome(&i.to_string()) {
                part_1_sum += i;
            }
            if has_repeat(&i.to_string()) {
                part_2_sum += i;
            }
        }
    }
    println!("Part 1: {}", part_1_sum);
    println!("Part 2: {}", part_2_sum);
}

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return false;
    }

    chars[0..chars.len() / 2] == chars[chars.len() / 2..]
}

fn has_repeat(s: &str) -> bool {
    for i in 1..s.len() {
        let mut to_repeat = s[0..i].to_string();
        while to_repeat.len() < s.len() {
            to_repeat = format!("{}{}", to_repeat, &s[0..i]);
        }
        if to_repeat == s {
            return true;
        }
    }
    false
}
