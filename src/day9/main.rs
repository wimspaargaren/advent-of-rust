use std::{cmp, fs};

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

struct Segment {
    start: usize,
    end: usize,
}

impl Segment {
    fn min_max_x(&self, points: &[Point]) -> (i64, i64) {
        if points[self.start].x < points[self.end].x {
            (points[self.start].x, points[self.end].x)
        } else {
            (points[self.end].x, points[self.start].x)
        }
    }

    fn min_max_y(&self, points: &[Point]) -> (i64, i64) {
        if points[self.start].y < points[self.end].y {
            (points[self.start].y, points[self.end].y)
        } else {
            (points[self.end].y, points[self.start].y)
        }
    }
}

struct Pair {
    a: usize,
    b: usize,
}

impl Pair {
    fn area(&self, points: &[Point]) -> i64 {
        let dist_x = (points[self.a].x - points[self.b].x).abs() + 1;
        let dist_y = (points[self.a].y - points[self.b].y).abs() + 1;
        dist_x * dist_y
    }

    fn min_max_x(&self, points: &[Point]) -> (i64, i64) {
        if points[self.a].x < points[self.b].x {
            (points[self.a].x, points[self.b].x)
        } else {
            (points[self.b].x, points[self.a].x)
        }
    }

    fn min_max_y(&self, points: &[Point]) -> (i64, i64) {
        if points[self.a].y < points[self.b].y {
            (points[self.a].y, points[self.b].y)
        } else {
            (points[self.b].y, points[self.a].y)
        }
    }
}

fn main() {
    let contents =
        fs::read_to_string("./src/day9/input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let points = get_points(lines);
    let (pairs, segments) = get_pairs_and_segments(&points);
    let mut part1 = 0;
    let mut part2 = 0;
    for pair in pairs {
        let area = pair.area(&points);
        if area > part1 {
            part1 = area;
        }
        if area <= part2 {
            continue;
        }
        let mut intersect = false;
        let (min_x, max_x) = pair.min_max_x(&points);
        let (min_y, max_y) = pair.min_max_y(&points);

        for segment in &segments {
            if point_intersect(min_x, max_x, min_y, max_y, segment, &points) {
                intersect = true;
                break;
            }
        }
        if intersect {
            continue;
        }
        if area > part2 {
            part2 = area;
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn point_intersect(
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    segment: &Segment,
    points: &[Point],
) -> bool {
    let (seg_min_x, seg_max_x) = segment.min_max_x(points);
    let (seg_min_y, seg_max_y) = segment.min_max_y(points);
    if points[segment.start].x == points[segment.end].x {
        if points[segment.start].x <= min_x || points[segment.start].x >= max_x {
            return false;
        }
        let overlap_start = cmp::max(min_y, seg_min_y);
        let overlap_end = cmp::min(max_y, seg_max_y);
        return overlap_start < overlap_end;
    }

    if points[segment.start].y <= min_y || points[segment.start].y >= max_y {
        return false;
    }
    let overlap_start = cmp::max(min_x, seg_min_x);
    let overlap_end = cmp::min(max_x, seg_max_x);
    overlap_start < overlap_end
}

fn get_pairs_and_segments(points: &[Point]) -> (Vec<Pair>, Vec<Segment>) {
    let mut pairs: Vec<Pair> = Vec::new();
    let mut segments: Vec<Segment> = Vec::new();
    for i in 0..points.len() {
        let cur = &points[i];
        let next = &points[(i + 1) % points.len()];
        if cur.x == next.x || cur.y == next.y {
            segments.push(Segment {
                start: i,
                end: (i + 1) % points.len(),
            });
        }
        for j in (i + 1)..points.len() {
            pairs.push(Pair { a: i, b: j });
        }
    }
    (pairs, segments)
}

fn get_points(lines: Vec<&str>) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    for line in lines {
        let splitted: Vec<&str> = line.split(',').collect();
        let x: i64 = splitted[0].parse().unwrap();
        let y: i64 = splitted[1].parse().unwrap();
        points.push(Point { x, y });
    }
    points
}
