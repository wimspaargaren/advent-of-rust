use std::{collections::HashMap, fs};

fn main() {
    let contents =
        fs::read_to_string("./src/day7/input.txt").expect("Should have been able to read the file");
    let mut grid = parse_grid(&contents);

    let mut recursive_store: HashMap<String, u64> = HashMap::new();
    let mut part1_visited: HashMap<String, bool> = HashMap::new();

    let (part1_count, part2_count) =
        walk_grid(&mut grid, &mut recursive_store, &mut part1_visited, 1);

    println!("Part 1 count: {part1_count}");
    println!("Part 2 count: {part2_count}");
}

fn walk_grid(
    grid: &mut Vec<Vec<&str>>,
    recursive_store: &mut HashMap<String, u64>,
    part1_visited: &mut HashMap<String, bool>,
    y_in: usize,
) -> (u64, u64) {
    let mut part1_count = 0;
    let mut part2_count = 0;

    for y in y_in..grid.len() - 1 {
        for x in 0..grid[y].len() - 1 {
            if !should_beam(grid[y - 1][x]) {
                continue;
            }
            match grid[y][x] {
                "." => {
                    grid[y][x] = "|";
                }
                "^" => {
                    match part1_visited.get(&map_key(x, y)) {
                        Some(_) => {}
                        None => {
                            part1_count += 1;
                            part1_visited.insert(map_key(x, y), true);
                        }
                    }
                    if has_left(x) {
                        let (part1_res, part2_res) =
                            recurse(grid, x - 1, y, recursive_store, part1_visited);
                        part1_count += part1_res;
                        part2_count += part2_res;
                    }
                    if has_right(x, grid[y].len()) {
                        let (part1_res, part2_res) =
                            recurse(grid, x + 1, y, recursive_store, part1_visited);
                        part1_count += part1_res;
                        part2_count += part2_res;
                    }
                    return (part1_count, part2_count);
                }
                _ => {}
            }
        }
    }

    (part1_count, part2_count + 1)
}

fn map_key(x: usize, y: usize) -> String {
    format!("{}-{}", x, y)
}

fn recurse(
    grid: &mut [Vec<&str>],
    x: usize,
    y: usize,
    recursive_store: &mut HashMap<String, u64>,
    part1_visited: &mut HashMap<String, bool>,
) -> (u64, u64) {
    let tile = grid[y][x];
    if tile != "." {
        return (0, 0);
    }
    match recursive_store.get(&map_key(x, y)) {
        Some(val) => (0, *val),
        None => {
            let mut new_grid = grid.to_vec();
            new_grid[y][x] = "|";
            let (part1_count, part2_count) =
                walk_grid(&mut new_grid, recursive_store, part1_visited, y + 1);
            recursive_store.insert(map_key(x, y), part2_count);
            (part1_count, part2_count)
        }
    }
}

fn has_left(n: usize) -> bool {
    n > 0
}

fn has_right(n: usize, len: usize) -> bool {
    n < len - 1
}

fn should_beam(tile: &str) -> bool {
    tile == "S" || tile == "|"
}

fn parse_grid(input: &str) -> Vec<Vec<&str>> {
    input.lines().map(|line| line.split("").collect()).collect()
}
