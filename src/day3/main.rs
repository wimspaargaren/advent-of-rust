use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./src/day3/input.txt").expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let mut total_part1: u64 = 0;
    let mut total_part2: u64 = 0;
    for line in lines {
        let mut digits = Vec::new();
        for c in line.chars() {
            let d = c.to_digit(10).unwrap() as u64;
            digits.push(d);
        }
        total_part1 += largest_number(digits.clone(), 2, String::new(), 0);
        total_part2 += largest_number(digits.clone(), 12, String::new(), 0);
    }
    println!("Part 1: {total_part1}");
    println!("Part 2: {total_part2}");
}

fn largest_number(numbers: Vec<u64>, total_length: u64, total: String, current_index: u64) -> u64 {
    let mut max_number = 0;
    let mut max_number_index = 0;
    let cur_len = total.len() as u64;
    let numbers_len = numbers.len() as u64;
    for i in current_index..(numbers_len - (total_length - cur_len - 1)) {
        if numbers[i as usize] > max_number {
            max_number = numbers[i as usize];
            max_number_index = i + 1;
        }
    }
    let new_total = total + &max_number.to_string();
    if new_total.len() as u64 == total_length {
        return new_total.parse::<u64>().unwrap();
    }
    largest_number(numbers, total_length, new_total, max_number_index)
}
