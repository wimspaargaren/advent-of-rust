use std::{cmp, collections::HashSet, fs};

struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    fn new(x: u32, y: u32, z: u32) -> Point {
        Point { x, y, z }
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x as i64 - other.x as i64;
        let dy = self.y as i64 - other.y as i64;
        let dz = self.z as i64 - other.z as i64;
        let temp = dx.pow(2) + dy.pow(2) + dz.pow(2);
        (temp as f64).sqrt()
    }
}

struct Pair {
    p1_idx: usize,
    p2_idx: usize,
    distance: f64,
}

struct Group {
    point_ids: HashSet<usize>,
}

impl Group {
    fn new() -> Group {
        Group {
            point_ids: HashSet::new(),
        }
    }

    fn add_point(&mut self, point_id: usize) {
        self.point_ids.insert(point_id);
    }
}

fn main() {
    let contents =
        fs::read_to_string("./src/day8/input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let points = get_points(lines);
    let pairs = get_pairs(&points);

    let mut individual_points = HashSet::new();
    let mut groups: Vec<Group> = Vec::new();

    let mut part_1 = 0;
    let mut part_2: u64 = 0;

    for (i, pair) in pairs.iter().enumerate() {
        individual_points.insert(pair.p1_idx);
        individual_points.insert(pair.p2_idx);
        if points.len() == individual_points.len() {
            part_2 = points[pair.p1_idx].x as u64 * points[pair.p2_idx].x as u64;
            break;
        }
        let mut index_group1 = None;
        let mut index_group2 = None;
        for (j, g) in groups.iter().enumerate() {
            if g.point_ids.contains(&pair.p1_idx) {
                index_group1 = Some(j);
            }
            if g.point_ids.contains(&pair.p2_idx) {
                index_group2 = Some(j);
            }
        }
        match (index_group1, index_group2) {
            (None, None) => {
                let mut new_group = Group::new();
                new_group.add_point(pair.p1_idx);
                new_group.add_point(pair.p2_idx);
                groups.push(new_group);
            }
            (Some(index_group1), None) => {
                groups[index_group1].add_point(pair.p2_idx);
            }
            (None, Some(index_group2)) => {
                groups[index_group2].add_point(pair.p1_idx);
            }
            (Some(index_group1), Some(index_group2)) => {
                if index_group1 == index_group2 {
                    continue;
                }
                let mut index_to_remove = index_group1;
                let mut index_to_merge = index_group2;
                if index_group2 > index_group1 {
                    index_to_remove = index_group2;
                    index_to_merge = index_group1;
                }
                let group_to_merge = groups.remove(index_to_remove);
                for p in group_to_merge.point_ids.iter() {
                    groups[index_to_merge].add_point(*p);
                }
            }
        }

        if i == 999 {
            groups.sort_by(|a, b| {
                if a.point_ids.len() < b.point_ids.len() {
                    return cmp::Ordering::Greater;
                } else if a.point_ids.len() > b.point_ids.len() {
                    return cmp::Ordering::Less;
                }
                cmp::Ordering::Equal
            });

            let mut res = 1;
            for group in groups.iter().take(3) {
                res *= group.point_ids.len();
            }
            part_1 = res as u32;
        }
    }
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

fn get_points(lines: Vec<&str>) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    for line in lines {
        let coords: Vec<u32> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
        let point = Point::new(coords[0], coords[1], coords[2]);
        points.push(point);
    }
    points
}

fn get_pairs(points: &[Point]) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = Vec::new();
    let mut added = HashSet::new();
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i == j {
                continue;
            }
            let key1 = format!("{}-{}", i, j);
            let key2 = format!("{}-{}", j, i);
            if added.contains(&key1) || added.contains(&key2) {
                continue;
            }
            added.insert(key1);
            added.insert(key2);
            let distance = points[i].distance(&points[j]);
            let pair = Pair {
                p1_idx: i,
                p2_idx: j,
                distance,
            };
            pairs.push(pair);
        }
    }
    pairs.sort_by(|a, b| {
        if a.distance < b.distance {
            return cmp::Ordering::Less;
        } else if a.distance > b.distance {
            return cmp::Ordering::Greater;
        }
        cmp::Ordering::Equal
    });

    pairs
}
