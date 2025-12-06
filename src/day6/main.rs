use std::fs;

struct Calc {
    values: Vec<String>,
    operator: String,
    column_length: usize,
}

fn main() {
    let contents =
        fs::read_to_string("./src/day6/input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let column_lengths = get_column_lengths(lines.clone());
    let calcs = get_calcs(lines, column_lengths);

    let mut part1 = 0;
    for calc in &calcs {
        part1 += get_total(calc.values.clone(), calc.operator.clone());
    }
    let mut part2 = 0;
    for calc in &calcs {
        let mut new_to_calc = vec![String::new(); calc.column_length];
        for val in &calc.values {
            for (i, c) in val.chars().enumerate() {
                if c == ' ' {
                    continue;
                }
                new_to_calc[i].push(c);
            }
        }
        part2 += get_total(new_to_calc, calc.operator.clone());
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn get_total(values: Vec<String>, operator: String) -> u64 {
    let mut total: u64 = 0;
    for val in values {
        let val = val.trim();
        let num: u64 = val.parse().expect("Failed to parse number");
        match operator.as_str() {
            "+" => total += num,
            "*" => {
                if total == 0 {
                    total = 1;
                }
                total *= num;
            }
            _ => panic!("unknown operator"),
        }
    }
    total
}

fn get_calcs(lines: Vec<&str>, column_lengths: Vec<usize>) -> Vec<Calc> {
    let mut calcs = Vec::new();
    for c_len in &column_lengths {
        calcs.push(Calc {
            values: Vec::new(),
            operator: String::new(),
            column_length: *c_len,
        });
    }

    for (line_index, line) in lines.iter().enumerate() {
        let mut start = 0;
        let new_line = line.to_string() + " ";
        for (i, c_len) in column_lengths.iter().enumerate() {
            let end = start + c_len + 1;
            if line_index == lines.len() - 1 {
                calcs[i].operator = new_line[start..end].trim().to_string();
            } else {
                calcs[i].values.push(new_line[start..end].to_string());
            }
            start = end;
        }
    }
    calcs
}

fn get_column_lengths(lines: Vec<&str>) -> Vec<usize> {
    let mut column_lengths = Vec::new();
    for (i_line, line) in lines.iter().enumerate() {
        if i_line == lines.len() - 1 {
            break;
        }

        let numbers: Vec<&str> = line.split_whitespace().collect();
        for (i, number) in numbers.iter().enumerate() {
            if i_line == 0 {
                column_lengths.push(number.len());
            } else if number.len() > column_lengths[i] {
                column_lengths[i] = number.len();
            }
        }
    }

    column_lengths
}
