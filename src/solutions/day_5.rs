use std::cmp::{max, min};
use std::collections::HashMap;

pub fn day_5_a() {
    let start = std::time::Instant::now();
    let mut count_map: HashMap<(i32, i32), u16> = HashMap::new();
    include_str!("../../input/day_5.txt")
        .lines()
        .map(line_to_path)
        .for_each(|path| {
            let ((x1, y1), (x2, y2)) = path;
            if x1 == x2 {
                for y in min(y1, y2)..=max(y1, y2) {
                    count_map.insert(
                        (x1, y),
                        count_map.get(&(x1, y)).unwrap_or(&0) + 1
                    );
                }
            } else if y1 == y2 {
                for x in min(x1, x2)..=max(x1, x2) {
                    count_map.insert(
                        (x, y1),
                        count_map.get(&(x, y1)).unwrap_or(&0) + 1
                    );
                }
            }
        });
    println!(
        "{} points. Elapsed time: {:?}.",
        count_map
            .values()
            .filter(|line_count| **line_count > 1)
            .count(),
        start.elapsed()
    );
}

pub fn day_5_b() {
    let start = std::time::Instant::now();
    let mut count_map: HashMap<(i32, i32), u16> = HashMap::new();
    include_str!("../../input/day_5.txt")
        .lines()
        .map(line_to_path)
        .for_each(|path| {
            let ((x1, y1), (x2, y2)) = path;
            if x1 == x2 {
                for y in min(y1, y2)..=max(y1, y2) {
                    count_map.insert(
                        (x1, y),
                        count_map.get(&(x1, y)).unwrap_or(&0) + 1
                    );
                }
            } else if y1 == y2 {
                for x in min(x1, x2)..=max(x1, x2) {
                    count_map.insert(
                        (x, y1),
                        count_map.get(&(x, y1)).unwrap_or(&0) + 1
                    );
                }
            } else {
                let x_step = if x2 > x1 { 1 } else { - 1 };
                let y_step = if y2 > y1 { 1 } else { - 1 };
                let (mut x, mut y) = (x1, y1);
                loop {
                    count_map.insert(
                        (x, y),
                        count_map.get(&(x, y)).unwrap_or(&0) + 1
                    );
                    if x == x2 && y == y2 {
                        break;
                    }
                    x += x_step;
                    y += y_step;
                }
            }
        });
    println!(
        "{} points. Elapsed time: {:?}.",
        count_map
            .values()
            .filter(|line_count| **line_count > 1)
            .count(),
        start.elapsed()
    );
}

fn line_to_path(line: &str) -> ((i32, i32), (i32, i32)) {
    let (a_str, b_str) = line.split_once(" -> ").unwrap();
    let (x1_str, y1_str) = a_str.split_once(',').unwrap();
    let (x2_str, y2_str) = b_str.split_once(',').unwrap();
    (
        (x1_str.parse().unwrap(), y1_str.parse().unwrap()),
        (x2_str.parse().unwrap(), y2_str.parse().unwrap())
    )
}