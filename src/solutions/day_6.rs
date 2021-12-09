pub fn all() {
    day_6_a();
    day_6_b();
}

pub fn day_6_a() {
    println!("6a: {}", solve(80));
}

pub fn day_6_b() {
    println!("6b: {}", solve(256));
}

fn solve(days: u16) -> u64 {
    let mut map = [0; 9];
    include_str!("../../input/day_6.txt")
        .split(',')
        .for_each(|fish| map[fish.parse::<usize>().unwrap()] += 1);
    (0..days)
        .for_each(|_| {
            map[7] += map[0];
            map.rotate_left(1);
        });
    map.iter().sum()
}