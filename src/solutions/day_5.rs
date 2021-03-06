use std::cmp::Ordering::*;

pub fn all() {
    day_5_a();
    day_5_b();
}

pub fn day_5_a() {
    println!("5a: {}", solve(false));
}

pub fn day_5_b() {
    println!("5b: {}", solve(true));
}

fn solve(include_diagonals: bool) -> usize {
    let mut map = [[0u16; 1000]; 1000];
    let mut count = 0;
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
                let current_value = map[x as usize][y as usize];
                if current_value == 1 {
                    count += 1;
                }
                map[x as usize][y as usize] = current_value + 1;
                if x == x2 && y == y2 {
                    break;
                }
                x += x_step;
                y += y_step;
            }
        });
    count
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
