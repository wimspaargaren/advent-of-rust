use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./src/day4/input.txt").expect("Should have been able to read the file");

    let mut grid = parse_grid(&contents);
    let part1_count = count_to_be_removed(&mut grid, false);
    let mut part2_count = 0;

    loop {
        let to_be_removed = count_to_be_removed(&mut grid, true);
        part2_count += to_be_removed;
        if to_be_removed == 0 {
            break;
        }
    }

    println!("Part 1 count: {}", part1_count);
    println!("Part 2 count: {}", part2_count);
}

fn count_to_be_removed(grid: &mut Vec<Vec<&str>>, with_removal: bool) -> u32 {
    let mut count = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] != "@" {
                continue;
            }
            let neighbors = adjacent_cells(x, y, grid);
            let mut neighbor_scrolls = 0;
            for neighbor in neighbors {
                if neighbor == "@" {
                    neighbor_scrolls += 1;
                }
            }
            if neighbor_scrolls < 4 {
                count += 1;
                if with_removal {
                    grid[x][y] = ".";
                }
            }
        }
    }
    count
}

fn adjacent_cells(x: usize, y: usize, grid: &[Vec<&str>]) -> Vec<String> {
    let mut neighbors = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            if new_x >= 0
                && new_x < grid.len() as isize
                && new_y >= 0
                && new_y < grid[0].len() as isize
            {
                neighbors.push(grid[new_x as usize][new_y as usize].to_string());
            }
        }
    }
    neighbors
}

fn parse_grid(input: &str) -> Vec<Vec<&str>> {
    input.lines().map(|line| line.split("").collect()).collect()
}
