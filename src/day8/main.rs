use std::{cmp, collections::HashMap, fs, hash::Hash};

#[derive(Eq, Hash, PartialEq, Clone)]
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
    point1: Point,
    point2: Point,
    distance: f64,
}

struct Group {
    points: HashMap<Point, bool>,
}

impl Group {
    fn new() -> Group {
        Group {
            points: HashMap::new(),
        }
    }

    fn add_point(&mut self, point: Point) {
        self.points.insert(point, true);
    }
}

fn main() {
    let contents =
        fs::read_to_string("./src/day8/input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let points = get_points(lines);
    let pairs = get_pairs(points.clone());

    let mut individual_points = HashMap::<Point, bool>::new();
    let mut groups: Vec<Group> = Vec::new();

    let mut part_1 = 0;
    let mut part_2: u64 = 0;

    for (i, pair) in pairs.iter().enumerate() {
        individual_points.insert(pair.point1.clone(), true);
        individual_points.insert(pair.point2.clone(), true);
        if points.len() == individual_points.len() {
            part_2 = pair.point1.x as u64 * pair.point2.x as u64;
            break;
        }
        let mut index_point1 = -1;
        let mut index_point2 = -1;
        for (j, g) in groups.iter().enumerate() {
            if g.points.contains_key(&pair.point1) {
                index_point1 = j as i32;
            }
            if g.points.contains_key(&pair.point2) {
                index_point2 = j as i32;
            }
        }
        if index_point1 == -1 && index_point2 == -1 {
            let mut new_group = Group::new();
            new_group.add_point(pair.point1.clone());
            new_group.add_point(pair.point2.clone());
            groups.push(new_group);
        } else if index_point1 != -1 && index_point2 == -1 {
            groups[index_point1 as usize].add_point(pair.point2.clone());
        } else if index_point1 == -1 && index_point2 != -1 {
            groups[index_point2 as usize].add_point(pair.point1.clone());
        } else if index_point1 != index_point2 {
            let group_to_merge = groups[index_point2 as usize].points.clone();
            for p in group_to_merge.keys() {
                groups[index_point1 as usize].add_point(p.clone());
            }
            groups.remove(index_point2 as usize);
        }
        if i == 999 {
            groups.sort_by(|a, b| {
                if a.points.len() < b.points.len() {
                    return cmp::Ordering::Greater;
                } else if a.points.len() > b.points.len() {
                    return cmp::Ordering::Less;
                }
                cmp::Ordering::Equal
            });

            let mut res = 1;
            for group in groups.iter().take(3) {
                res *= group.points.len();
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

fn get_pairs(points: Vec<Point>) -> Vec<Pair> {
    let mut pairs: Vec<Pair> = Vec::new();
    let mut added = HashMap::new();
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i == j {
                continue;
            }
            let key1 = format!("{}-{}", i, j);
            let key2 = format!("{}-{}", j, i);
            if added.contains_key(&key1) || added.contains_key(&key2) {
                continue;
            }
            added.insert(key1, true);
            added.insert(key2, true);
            let distance = points[i].distance(&points[j]);
            let pair = Pair {
                point1: Point::new(points[i].x, points[i].y, points[i].z),
                point2: Point::new(points[j].x, points[j].y, points[j].z),
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
