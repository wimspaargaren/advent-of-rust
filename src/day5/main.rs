use std::fs;

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, id: u64) -> bool {
        id >= self.start && id <= self.end
    }
}

fn main() {
    let contents =
        fs::read_to_string("./src/day5/input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut ranges_done = false;
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    for line in lines {
        if !ranges_done {
            if line.is_empty() {
                ranges_done = true;
                continue;
            }
            let splitted = line.split("-").collect::<Vec<&str>>();
            let range = Range {
                start: splitted[0].parse().unwrap(),
                end: splitted[1].parse().unwrap(),
            };
            ranges.push(range);
            continue;
        }
        let id: u64 = line.parse().unwrap();
        ids.push(id);
    }

    let mut part_1_count = 0;

    for id in ids {
        for range in &ranges {
            if range.contains(id) {
                part_1_count += 1;
                break;
            }
        }
    }

    println!("Part 1: {part_1_count}");

    let merged_ranges = merge_ranges(&mut ranges);
    let mut part_2_count = 0;
    for range in merged_ranges {
        part_2_count += range.end - range.start + 1;
    }

    println!("Part 2: {part_2_count}");
}

fn merge_ranges(ranges: &mut [Range]) -> Vec<Range> {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut merged: Vec<Range> = Vec::new();

    for range in ranges.iter() {
        if let Some(last) = merged.last_mut() {
            if range.start <= last.end {
                last.end = last.end.max(range.end);
            } else {
                merged.push(Range {
                    start: range.start,
                    end: range.end,
                });
            }
        } else {
            merged.push(Range {
                start: range.start,
                end: range.end,
            });
        }
    }

   merged
}
