use std::cmp::Ordering::*;
use std::collections::HashMap;

pub fn day_5_a() {
    let start = std::time::Instant::now();
    let count = solve(false);
    println!("{} points. Elapsed time: {:?}.", count, start.elapsed());
}

pub fn day_5_b() {
    let start = std::time::Instant::now();
    let count = solve(true);
    println!("{} points. Elapsed time: {:?}.", count, start.elapsed());
}

fn solve(include_diagonals: bool) -> usize {
    let mut count_map: HashMap<(i32, i32), u16> = HashMap::new();
    include_str!("../../input/day_5.txt")
        .lines()
        .map(line_to_path)
        .for_each(|path| {
            let ((mut x, mut y), (x2, y2)) = path;
            if !(include_diagonals || x == x2 || y == y2) {
                return;
            }
            let x_step = get_step(x, x2);
            let y_step = get_step(y, y2);
            loop {
                count_map.insert((x, y), count_map.get(&(x, y)).unwrap_or(&0) + 1);
                if x == x2 && y == y2 {
                    break;
                }
                x += x_step;
                y += y_step;
            }
        });
    count_map
        .values()
        .filter(|line_count| **line_count > 1)
        .count()
}

fn line_to_path(line: &str) -> ((i32, i32), (i32, i32)) {
    let (a_str, b_str) = line.split_once(" -> ").unwrap();
    let (x1_str, y1_str) = a_str.split_once(',').unwrap();
    let (x2_str, y2_str) = b_str.split_once(',').unwrap();
    (
        (x1_str.parse().unwrap(), y1_str.parse().unwrap()),
        (x2_str.parse().unwrap(), y2_str.parse().unwrap()),
    )
}

fn get_step(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Equal => 0,
        Less => 1,
        Greater => -1,
    }
}
